use crate::renderer::get_renderer_for;
use crate::{Irqs, SharedBuffer, SharedEngine};

use defmt::*;

use cyw43::{Control, JoinOptions};
use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_net::{Config, Stack, StackResources};
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_rp::clocks::RoscRng;
use embassy_rp::peripherals::{DMA_CH1, PIN_24, PIN_29, PIO1};
use embassy_rp::pio::Pio;
use embassy_time::{Duration, Timer};

use rand::RngCore;
use render_engine::fixedcolor::FixedColor;
use static_cell::StaticCell;
use command::Command;

const WIFI_NETWORK: &str = "18mlf";
const WIFI_PASSWORD: &str = "eieioitsofftoworkwego";

#[embassy_executor::task]
async fn cyw43_task(runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO1, 0, DMA_CH1>>) -> ! {
    info!("cy243_task");
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn io_task(stack: Stack<'static> , mut control: Control<'static>, engine: &'static SharedEngine, buffer: &'static SharedBuffer) {

    loop {
        match control
            .join(WIFI_NETWORK, JoinOptions::new(WIFI_PASSWORD.as_bytes()))
            .await
        {
            Ok(_) => break,
            Err(err) => {
                info!("join failed with status={}", err.status);
            }
        }
    }

    // Wait for DHCP, not necessary when using static IP``
    info!("waiting for DHCP...");
    while !stack.is_config_up() {
        Timer::after_millis(100).await;
    }
    info!("DHCP is now up!");
    let c = stack.config_v4().unwrap();
    info!("IP: {:?}", c.address);


    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let mut buf = [0; 4096];


    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));
        
        control.gpio_set(0, false).await;
        info!("Listening on TCP:1234...");
        if let Err(e) = socket.accept(1234).await {
            warn!("accept error: {:?}", e);
                continue;
        }

        info!("Received connection from {:?}", socket.remote_endpoint());
        control.gpio_set(0, true).await;



        loop {
            let n = match socket.read(&mut buf).await {
                Ok(0) => {
                    warn!("read EOF");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    warn!("read error: {:?}", e);
                    break;
                }
            };

            let command: Command = minicbor_serde::from_slice(&buf[..n]).unwrap();
            match command {
                Command::Animate(anim) => {
                    engine.lock(|engine| {
                        engine.borrow_mut().set_renderer(get_renderer_for(anim));
                    }); 
                }
                Command::Clear(r,g,b) => {
                    buffer.lock(|buffer| {
                        let color = FixedColor::from_rgb8(r, g, b);
                        info!("Clearing the display to {} {} {}", color.r.to_num::<f32>(), color.g.to_num::<f32>(), color.b.to_num::<f32>());
                        buffer.borrow_mut().get_mut_buffer().clear_to_color(FixedColor::from_rgb8(r, g, b));
                    });
                }
                Command::Flush => {
                    // buffer.lock(|buffer| {
                    //     let mut b = buffer.borrow_mut();
                    //     engine.lock(|engine| {
                    //         engine.borrow_mut().render(Fixed::ZERO, Fixed::ZERO, b.get_mut_buffer());
                    //     });
                    //     ws2812.write_iter(b.into_iter());
                    // });
                }
            }
        }
    }
}



pub async fn init_wifi(spawner: Spawner, pwr_pin: AnyPin, cs_pin: AnyPin, pio: PIO1, dio: PIN_24, clk: PIN_29, dma: DMA_CH1, engine: &'static SharedEngine, buffer: &'static SharedBuffer) {
    info!("wifi task");
    let mut rng = RoscRng;

    let fw = include_bytes!("../../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../../cyw43-firmware/43439A0_clm.bin");

    let pwr = Output::new(pwr_pin, Level::Low);
    let cs = Output::new(cs_pin, Level::High);

    let mut pio = Pio::new(pio, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        dio,
        clk,
        dma,
    );


    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());

    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;

    spawner.spawn(cyw43_task(runner)).unwrap();

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let config = Config::dhcpv4(Default::default());
    //let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
    //    address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 69, 2), 24),
    //    dns_servers: Vec::new(),
    //    gateway: Some(Ipv4Address::new(192, 168, 69, 1)),
    //});

    let seed = rng.next_u64();

    // Init network stack
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let (stack, runner) =
        embassy_net::new(net_device, config, RESOURCES.init(StackResources::new()), seed);

    spawner.spawn(net_task(runner)).unwrap();

    spawner.spawn(io_task(stack, control, engine, buffer)).unwrap();
}
