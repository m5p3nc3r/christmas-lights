#![no_std]
#![no_main]

mod renderer;
//mod statusled;
mod wifi;

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::{PIO0, PIO1};
use embassy_rp::pio::InterruptHandler;
use embassy_sync::blocking_mutex::{Mutex, raw::CriticalSectionRawMutex};

use renderer::{led_strip_control, Buffer50x24, RenderEngine50x24};
use static_cell::StaticCell;
use core::cell::RefCell;
use wifi::init_wifi;
//use crate::statusled::status_led;
use crate::renderer::render_engine;

use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(pub struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    PIO1_IRQ_0 => InterruptHandler<PIO1>;
});

pub type SharedBuffer = Mutex<CriticalSectionRawMutex, RefCell<Buffer50x24>>;
static BUFFER: StaticCell<SharedBuffer> = StaticCell::new();

pub type SharedEngine = Mutex<CriticalSectionRawMutex, RefCell<RenderEngine50x24>>;
static ENGINE: StaticCell<SharedEngine> = StaticCell::new(); 

#[embassy_executor::main]
async fn main(spawner: Spawner) {   
    info!("Start");
    let p: embassy_rp::Peripherals = embassy_rp::init(Default::default());

    let buffer = BUFFER.init(Mutex::new(RefCell::new(Buffer50x24::new())));
    let engine = ENGINE.init(Mutex::new(RefCell::new(RenderEngine50x24::new())));


    init_wifi(spawner, p.PIN_23.into(), p.PIN_25.into(), p.PIO1, 
        p.PIN_24.into(), p.PIN_29.into(), p.DMA_CH1, engine, buffer).await;

    // There is no programmable LED on the Pico W
    //spawner.spawn(status_led(p.PIN_25.into())).unwrap();
    spawner.spawn(led_strip_control(p.PIO0, p.DMA_CH0, p.PIN_16, buffer)).unwrap();
    spawner.spawn(render_engine(engine, buffer)).unwrap();
}

 
