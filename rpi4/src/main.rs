use render_engine::{RenderBuffer, RenderEngine, Fixed, Renderer, RenderType};
use smart_leds::{SmartLedsWrite, RGB};
use ws281x_rpi::Ws2812Rpi;

use std::{thread, time};


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

            let (r, g, b) = self.buffer.get_pixel(x, y).as_rgb8();

            self.index += 1;
            Some(RGB::new(r, g, b))
    
        } else {
            None
        }
    }
}

const PIN: i32 = 10;
const NUM_LEDS: usize = NUM_DROPS * LEDS_PER_DROP;
const DELAY: time::Duration = time::Duration::from_millis(1000);



fn main() {
    println!("Hello, world!");
    let mut buffer= Buffer50x24::new();
    let mut engine = RenderEngine::new();
    let sleep_duration = time::Duration::from_millis(40);

    engine.set_renderer(Renderer::Basic(RenderType::Snow));


    let mut ws = Ws2812Rpi::new(NUM_LEDS as i32, PIN).unwrap();


    loop {
        engine.render(Fixed::ZERO, Fixed::ZERO, buffer.get_mut_buffer());
        ws.write(buffer.into_iter());
        // sleep for 40ms
        thread::sleep(sleep_duration);
    }
}
