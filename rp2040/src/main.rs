#![no_std]
#![no_main]

mod ws2812;

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::Timer;

use render_engine::{RenderBuffer, RenderEngine, Fixed};
use smart_leds::RGB;

use crate::ws2812::Ws2812;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});


const LEDS_PER_DROP: usize = 24;
const NUM_DROPS: usize = 50;

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
            let x = self.index % size.x;
            let y = self.index / size.x;

            let (r, g, b) = self.buffer.get_pixel(x, y).to_rgb8();

            self.index += 1;
            return Some(RGB::new(r, g, b));
    
        } else {
            return None;
        }
    }
}


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Start");
    let p = embassy_rp::init(Default::default());

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    // This is the number of leds in the string. Helpfully, the sparkfun thing plus and adafruit
    // feather boards for the 2040 both have one built in.
    //    let mut data = [RGB8::default(); NUM_LEDS];

    // Common neopixel pins:
    // Thing plus: 8
    // Adafruit Feather: 16;  Adafruit Feather+RFM95: 4
    let mut ws2812: Ws2812<'_, PIO0, 0, {NUM_DROPS * LEDS_PER_DROP}> = Ws2812::new(&mut common, sm0, p.DMA_CH0, p.PIN_16);
    let mut buffer= Buffer50x24::new();
    let mut engine = RenderEngine::new();


    loop {
        engine.render(Fixed::ZERO, Fixed::from_num(0.04), buffer.get_mut_buffer());
        ws2812.write(buffer.into_iter()).await;
        
        Timer::after_millis(40).await;
    }
}

 
