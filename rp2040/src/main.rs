#![no_std]
#![no_main]

mod ws2812;
mod statusled;
mod wifi;


use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::{DMA_CH0, PIN_16, PIO0, PIO1};
use embassy_rp::pio::{InterruptHandler, Pio};


use render_engine::{RenderBuffer, RenderEngine, Fixed, Renderer, RenderType};
use smart_leds::RGB;
use wifi::init_wifi;

use crate::ws2812::Ws2812;
use crate::statusled::status_led;





use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(pub struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    PIO1_IRQ_0 => InterruptHandler<PIO1>;
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

            let (r, g, b) = self.buffer.get_pixel(x, y).as_rgb8();

            self.index += 1;
            Some(RGB::new(r, g, b))
    
        } else {
            None
        }
    }
}

#[embassy_executor::task]
pub async fn display_engine(pio: PIO0, dma: DMA_CH0, pin: PIN_16) {
    let Pio {
        mut common, sm0, ..
    } = Pio::new(pio, Irqs);

     let mut ws2812: Ws2812<'_, _, 0, {NUM_DROPS * LEDS_PER_DROP}> = Ws2812::new(&mut common, sm0, dma, pin);
     let mut buffer= Buffer50x24::new();
     let mut engine = RenderEngine::<{NUM_DROPS * LEDS_PER_DROP}, NUM_DROPS, LEDS_PER_DROP>::new();

     engine.set_renderer(Renderer::Basic(RenderType::Snow));

    loop {
        engine.render(Fixed::ZERO, Fixed::ZERO, buffer.get_mut_buffer());
        ws2812.write(buffer.into_iter()).await;
        
        info!("Still alive");
        //Timer::after(Duration::from_millis(40)).await;
    }
}





#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Start");
    let p: embassy_rp::Peripherals = embassy_rp::init(Default::default());

    init_wifi(spawner, 
            p.PIN_23.into(), 
            p.PIN_25.into(), 
            p.PIO1, 
            p.PIN_24.into(), 
            p.PIN_29.into(),
            p.DMA_CH1
        ).await;




   // spawner.spawn(status_led(p.PIN_25.into())).unwrap();
    //spawner.spawn(display_engine(p.PIO0, p.DMA_CH0, p.PIN_16)).unwrap();

//     let Pio {
//         mut common, sm0, ..
//     } = Pio::new(p.PIO0, Irqs);


//     let mut ws2812: Ws2812<'_, _, 0, {NUM_DROPS * LEDS_PER_DROP}> = Ws2812::new(&mut common, sm0, p.DMA_CH0, p.PIN_16);
//     let mut buffer= Buffer50x24::new();
//     let mut engine = RenderEngine::<{NUM_DROPS * LEDS_PER_DROP}, NUM_DROPS, LEDS_PER_DROP>::new();

//     engine.set_renderer(Renderer::Basic(RenderType::Snow));

//     loop {
//         engine.render(Fixed::ZERO, Fixed::ZERO, buffer.get_mut_buffer());
//         ws2812.write(buffer.into_iter()).await;
        
//         //info!("Still alive");
//         //Timer::after(Duration::from_millis(40)).await;
//     }
}

 
