//mod stream_frame;

use std::io::prelude::*;
use std::fs::File;
use std::net::{Ipv4Addr, TcpStream};



use clap::Parser;
use image::imageops::resize;
use image::{ImageBuffer, ImageReader};
use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;
use command::Command as StreamCommand;





#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    ip: Ipv4Addr,
    #[clap(short, long)]
    port: u16,
    #[clap(subcommand)]
    command: Command,
}


#[derive(clap::Subcommand)]
enum Command {
    Clear(ClearArgs),
    Animate(AnimateArgs),
    Flush,
}

#[derive(clap::Args)]
struct ClearArgs {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(clap::Args)]
struct AnimateArgs {
    #[clap(subcommand)]
    subcommand: Animation,
}

#[derive(clap::Subcommand, Debug)]
enum Animation {
    None,
    Snow,
    Sparkle,
    Rainbow
}


impl Into<command::Animation> for Animation {
    fn into(self) -> command::Animation {
        match self {
            Animation::None => command::Animation::None,
            Animation::Snow => command::Animation::Snow,
            Animation::Sparkle => command::Animation::Sparkle,
            Animation::Rainbow => command::Animation::Rainbow,
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cli = Cli::parse();

    let mut stream = TcpStream::connect("192.168.1.213:1234")?;

    let command = match cli.command {
        Command::Clear(args) => {
            println!("Clearing the display to {} {} {}", args.r, args.g, args.b);
            StreamCommand::Clear(args.r, args.g, args.b)
        }
        
        Command::Animate(args) => {
            println!("Animating with {:?}", args.subcommand);
            StreamCommand::Animate(args.subcommand.into())
        }
        Command::Flush => {
            println!("Flushing the display");
            StreamCommand::Flush
        }
    };

    let buf = minicbor_serde::to_vec(&command)?;
    stream.write(&buf)?;
    stream.flush()?;


    // let mut stream = TcpStream::connect("192.168.1.213:1234")?;
    // let command = command::Command::Clear(255, 0, 0);

    // let mut buf = minicbor_serde::to_vec(&command)?;
    // stream.write(&buf)?;
    // stream.flush()?;


    // println!("fps = {:?}", cli.fps.unwrap_or(10));

    // let reader = ImageReader::open(cli.source.clone())?;
    // let format = reader.format().ok_or("unknown image format")?;


    // if format == image::ImageFormat::Gif {
    //     println!("GIF");
    //     // Special case of GIF
    //     let path = std::path::Path::new(&cli.source);
    //     let file = File::open(path)?;
    //     let decoder = GifDecoder::new(std::io::BufReader::new(file))?;
    //     let frames = decoder.into_frames().collect_frames()?;
    //     loop {
    //         for (index, frame) in frames.iter().enumerate() {
    //             println!("Frame {}", index);
    //             let buffer = frame.buffer();
    //             let resized = resize(buffer, cli.x, cli.y, image::imageops::FilterType::CatmullRom);


    //             send_frame(resized)?;
    //         }   
    //     }
    // } else {
    //     println!("Not GIF");
    //     let image = reader.decode()?;
    //     let buffer = image.to_rgba8();
    //     let resized = resize(&buffer, cli.x, cli.y, image::imageops::FilterType::CatmullRom);
    //     send_frame(resized);
    // }

    Ok(())
}
// fn send_frame(buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> std::io::Result<()> {
//     let mut stream = TcpStream::connect("192.168.1.213:1234")?;
//     // Send the frame to the server
//     for p in buffer.pixels() {
//         stream.write(&[p[0], p[1], p[2]])?;
//         println!("{:?}", p);
//     }
//     Ok(())
// }
