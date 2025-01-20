use crate::Irqs;

use embassy_rp::peripherals::{DMA_CH0, PIN_16, PIO0};
use embassy_rp::pio::Pio;
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_time::{Duration, Timer};

use render_engine::{RenderBuffer, RenderEngine, Fixed, Renderer, RenderType};
use smart_leds::RGB;

const LEDS_PER_DROP: usize = 24;
const NUM_DROPS: usize = 5;

struct Buffer50x24(RenderBuffer<{NUM_DROPS * LEDS_PER_DROP}, NUM_DROPS, LEDS_PER_DROP>);

impl Buffer50x24 {
    fn new() -> Self {
        Buffer50x24(RenderBuffer::new())
    }

    fn get_mut_buffer(&mut self) -> &mut RenderBuffer<{NUM_DROPS * LEDS_PER_DROP}, NUM_DROPS, LEDS_PER_DROP> {
        &mut self.0
    }
}

struct BufferIterator<'a> {
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

#[embassy_executor::task]
pub async fn render_engine(pio: PIO0, dma: DMA_CH0, pin: PIN_16) {
    let Pio { mut common, sm0, .. } = Pio::new(pio, Irqs);

    let program = PioWs2812Program::new(&mut common);
    let mut ws2812: PioWs2812<'_, _, 0, {NUM_DROPS * LEDS_PER_DROP}> = PioWs2812::new(&mut common, sm0, dma, pin, &program);

    let mut buffer= Buffer50x24::new();
    let mut engine = RenderEngine::<{NUM_DROPS * LEDS_PER_DROP}, NUM_DROPS, LEDS_PER_DROP>::new();

    engine.set_renderer(Renderer::Basic(RenderType::Rainbow));

    loop {
        engine.render(Fixed::ZERO, Fixed::ZERO, buffer.get_mut_buffer());
        ws2812.write_iter(buffer.into_iter()).await;        
        Timer::after(Duration::from_millis(40)).await;
    }
}
