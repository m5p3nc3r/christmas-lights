#![no_std]
#![no_main]

mod ws2812;

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::{InterruptHandler, Pio};
//use embassy_time::Timer;

use embedded_graphics::primitives::Triangle;
use embedded_graphics::{
    pixelcolor::*,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use smart_leds_matrix::layout::Layout;
use smart_leds_matrix::SmartLedMatrix;

use crate::ws2812::Ws2812;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

// Input a value 0 to 255 to get a color value
// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> Rgb888 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return Rgb888::new(255 - wheel_pos * 3, 0, wheel_pos * 3);
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return Rgb888::new(0, wheel_pos * 3, 255 - wheel_pos * 3);
    }
    wheel_pos -= 170;
    Rgb888::new(wheel_pos * 3, 255 - wheel_pos * 3, 0)
}

// Implement a specific layout for the led matrix.
// Configured with the number of drops and leds per drop.
// So with 4 drops of 4 leds each, and a zero at top left
// the layout would be:
// 0  4  8 12
// 1  5  9 13
// 2  6 10 14
// 3  7 11 15

struct LedDrops {
    width: u32,
    height: u32,
}

impl LedDrops {
    fn new(num_drops: usize, leds_per_drop: usize) -> Self {
        Self {
            width: num_drops as u32,
            height: leds_per_drop as u32,
        }
    }
}

impl Layout for LedDrops {
    fn map(&self, p: Point) -> Option<usize> {
        if p.x < 0 || p.y < 0 || p.x >= self.width as i32 || p.y >= self.height as i32 {
            return None;
        }

        let pos = p.x as u32 * self.height + p.y as u32;

        Some(pos as usize)
    }

    fn size(&self) -> Size {
        Size::new(self.width, self.height)
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
    const LEDS_PER_DROP: usize = 30;
    const NUM_DROPS: usize = 8;
    const NUM_LEDS: usize = LEDS_PER_DROP * NUM_DROPS;
    //    let mut data = [RGB8::default(); NUM_LEDS];

    // Common neopixel pins:
    // Thing plus: 8
    // Adafruit Feather: 16;  Adafruit Feather+RFM95: 4
    let ws2812 = Ws2812::new(&mut common, sm0, p.DMA_CH0, p.PIN_16);

    let mut matrix: SmartLedMatrix<Ws2812<'_, PIO0, 0, NUM_LEDS>, LedDrops, NUM_LEDS> =
        SmartLedMatrix::new(
            ws2812,
            //Rectangular::new(LEDS_PER_DROP as u32, NUM_DROPS as u32),
            LedDrops::new(NUM_DROPS, LEDS_PER_DROP),
        );

    matrix.set_brightness(64);
    let _ = matrix.clear(Rgb888::new(0, 0, 0));

    loop {
        for j in 0..255 as u8 {
            let colour = wheel(j);
            let fill_colour = wheel(((j as u16 + 128) & 255) as u8);
            let _ = Rectangle::new(Point::new(0, 0), Size::new(24, 24))
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .fill_color(fill_colour)
                        .stroke_color(colour)
                        .stroke_width(1)
                        .build(),
                )
                .draw(&mut matrix);

            // let _ = Triangle::new(Point::new(4, 8), Point::new(6, 16), Point::new(1, 16))
            //     .into_styled(
            //         PrimitiveStyleBuilder::new()
            //             .fill_color(Rgb888::GREEN)
            //             .build(),
            //     )
            //     .draw(&mut matrix);
            // Trigger the actual frame update on the matrix with gamma correction.
            let _ = matrix.flush_with_gamma();
        }
    }

    // Loop forever making RGB values and pushing them out to the WS2812.
    // loop {
    //     for j in 0..(256 * 5) {
    //         //            debug!("New Colors:");
    //         for i in 0..NUM_LEDS {
    //             data[i] = wheel(
    //                 (((i * 256) as u16 / (NUM_LEDS / NUM_DROPS) as u16 + j as u16) & 255) as u8,
    //             );
    //             // debug!("R: {} G: {} B: {}", data[i].r, data[i].g, data[i].b);
    //         }

    //         let swizzle = |pos: usize| -> usize { pos };

    //         ws2812.write(&data, swizzle).await;

    //         Timer::after_millis(10).await;
    //     }
    // }
}
