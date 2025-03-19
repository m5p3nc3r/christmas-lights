use crate::{Irqs, SharedBuffer, SharedEngine};

use defmt::info;
use embassy_futures::select::{select, Either};
use embassy_rp::peripherals::{DMA_CH0, PIN_16, PIO0};
use embassy_rp::pio::Pio;
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Ticker, Timer};

use render_engine::{RenderBuffer, RenderEngine, Renderer, RenderType};
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

impl Default for Buffer50x24 {
    fn default() -> Self {
        Self::new()
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


static RENDERENGINE_CONTROL: Channel<CriticalSectionRawMutex, Renderer, 2> = Channel::new();

pub async fn set_renderer(renderer: Renderer) {
    defmt::info!("Sending renderer control message");
    RENDERENGINE_CONTROL.send(renderer).await;
}

#[embassy_executor::task]
pub async fn render_engine(engine: &'static SharedEngine, buffer: &'static SharedBuffer) {
    engine.lock(|engine| {
//        engine.borrow_mut().set_renderer(Renderer::None);
        engine.borrow_mut().set_renderer(Renderer::Basic(RenderType::Snow));
    });

    let mut ticker = Ticker::every(Duration::from_millis(40));
    let mut paused = false;

    loop {
        match select(RENDERENGINE_CONTROL.receive(), ticker.next()).await {
            Either::First(r) => { // The control channel has received a message
                defmt::info!("Received renderer control message");
                engine.lock(|engine| {
                    engine.borrow_mut().set_renderer(r);
                });
                            
                paused = r == Renderer::None;
            }

            Either::Second(_) => { // The timer has expired
                if !paused {
                    // Get access to the shared render buffer
                    buffer.lock(|buffer| {
                        let mut b = buffer.borrow_mut();
                        engine.lock(|engine| {
                            engine.borrow_mut().render(0.0, 0.0, b.get_mut_buffer());
                        });
                    });
                
                    flush_led_strip().await;
                }
            }
        }


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