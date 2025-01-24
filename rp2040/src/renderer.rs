use crate::{Irqs, SharedBuffer, SharedEngine};

use embassy_rp::peripherals::{DMA_CH0, PIN_16, PIO0};
use embassy_rp::pio::Pio;
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};

use render_engine::{RenderBuffer, RenderEngine, Fixed, Renderer, RenderType};
use smart_leds::RGB;

const LEDS_PER_DROP: usize = 24;
const NUM_DROPS: usize = 5;

pub type RenderEngine50x24 = RenderEngine<{NUM_DROPS * LEDS_PER_DROP}, NUM_DROPS, LEDS_PER_DROP>;
pub struct Buffer50x24(RenderBuffer<{NUM_DROPS * LEDS_PER_DROP}, NUM_DROPS, LEDS_PER_DROP>);

impl Buffer50x24 {
    pub fn new() -> Self {
        Buffer50x24(RenderBuffer::new())
    }

    pub fn get_mut_buffer(&mut self) -> &mut RenderBuffer<{NUM_DROPS * LEDS_PER_DROP}, NUM_DROPS, LEDS_PER_DROP> {
        &mut self.0
    }
}

pub struct BufferIterator<'a> {
    buffer: &'a RenderBuffer<{NUM_DROPS * LEDS_PER_DROP}, NUM_DROPS, LEDS_PER_DROP>,
    index: u32,
}

impl<'a> IntoIterator for &'a Buffer50x24 {
    type Item = RGB<u8>;
    type IntoIter = BufferIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BufferIterator {
            buffer: &self.0,
            index: 0,
        }
    }
}
  
impl Iterator for BufferIterator<'_> {
    type Item = RGB<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let size = self.buffer.size();
        let len = size.x * size.y;
        if self.index < len {
            let y = self.index % size.y;
            let x = self.index / size.y;

            let (r, g, b) = self.buffer.get_pixel(x, y).as_rgb8();

            self.index += 1;
            Some(RGB::new(r, g, b))
    
        } else {
            None
        }
    }
}


pub fn get_renderer_for(command: command::Animation) -> Renderer {
    match command {
        command::Animation::None => Renderer::None,
        command::Animation::Snow => Renderer::Basic(RenderType::Snow),
        command::Animation::Sparkle => Renderer::Basic(RenderType::Sparkle),
        command::Animation::Rainbow => Renderer::Basic(RenderType::Rainbow),
    }
}

#[embassy_executor::task]
pub async fn render_engine(engine: &'static SharedEngine, buffer: &'static SharedBuffer) {
    engine.lock(|engine| {
        engine.borrow_mut().set_renderer(Renderer::Basic(RenderType::Snow));
    });

    loop {
        // Get access to the shared render buffer
        buffer.lock(|buffer| {
            let mut b = buffer.borrow_mut();
            engine.lock(|engine| {
                engine.borrow_mut().render(Fixed::ZERO, Fixed::ZERO, b.get_mut_buffer());
            });
        });
    
        flush_led_strip().await;

        Timer::after(Duration::from_millis(40)).await;
    }
}

static LEDSTRIP: Channel<CriticalSectionRawMutex, (), 2> = Channel::new();

pub async fn flush_led_strip() {
    LEDSTRIP.send(()).await;
}

#[embassy_executor::task]
pub async fn led_strip_control(pio: PIO0, dma: DMA_CH0, pin: PIN_16, buffer: &'static SharedBuffer) {
    let Pio { mut common, sm0, .. } = Pio::new(pio, Irqs);

    let program = PioWs2812Program::new(&mut common);
    let mut ws2812: PioWs2812<'_, _, 0, {NUM_DROPS * LEDS_PER_DROP}> = PioWs2812::new(&mut common, sm0, dma, pin, &program);

    loop {
        let _ = LEDSTRIP.receive().await;

        buffer.lock(|buffer| {
            let b = buffer.borrow();
            ws2812.write_iter(b.into_iter());
        });

        ws2812.flush().await;
    }
}