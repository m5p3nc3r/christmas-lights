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
    Display(DisplayArgs),
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

#[derive(clap::Args)]
#[derive(Clone)]
struct DisplayArgs {
    #[clap(short, long)]
    x: u32,
    #[clap(short, long)]
    y: u32,
    #[clap(short, long)]
    source: String,
    #[clap(short, long)]
    fps: Option<u32>,
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

    let mut stream = TcpStream::connect("192.168.1.214:1234")?;

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
        Command::Display(args ) => {
            display(&mut stream, args)?;
            StreamCommand::Flush
        }
    };

    let buf = minicbor_serde::to_vec(&command)?;
    stream.write(&buf)?;
    stream.flush()?;

    Ok(())
}


fn display(stream: &mut TcpStream, args: DisplayArgs) -> Result<(), Box<dyn std::error::Error>> {
    let reader = ImageReader::open(args.source.clone())?;
    let format = reader.format().ok_or("unknown image format")?;


    if format == image::ImageFormat::Gif {
        println!("GIF");
        // Special case of GIF
        let path = std::path::Path::new(&args.source);
        let file = File::open(path)?;
        let decoder = GifDecoder::new(std::io::BufReader::new(file))?;
        let frames = decoder.into_frames().collect_frames()?;
        let x = args.x;
        let y = args.y;
        loop {
            for (index, frame) in frames.iter().enumerate() {
                println!("Frame {}", index);
                let buffer = frame.buffer();
                let resized = resize(buffer, x, y, image::imageops::FilterType::CatmullRom);
                send_frame(stream, args.clone(), resized)?;
                std::thread::sleep(std::time::Duration::from_millis(500));
            }   
        }
    } else {
        println!("Not GIF");
        let image = reader.decode()?;
        let buffer = image.to_rgba8();
        let resized = resize(&buffer, args.x, args.y, image::imageops::FilterType::CatmullRom);
        send_frame(stream, args,  resized)?;
    }

    Ok(())

}

fn send_frame(stream: &mut TcpStream, args: DisplayArgs, buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
    send_command(stream, StreamCommand::Clear(0, 0, 0))?;
    // Send the frame to the server
    for (i, p) in buffer.pixels().enumerate() {
        let x = i as u32 % args.x;
        let y = i as u32 / args.x;

        let set_pixel = StreamCommand::SetPixel(x as u8, y as u8, p[0], p[1], p[2]);

        send_command(stream, set_pixel)?;
    }
    send_command(stream, StreamCommand::Flush)?;

    Ok(())
}

fn send_command(stream: &mut TcpStream, command: StreamCommand) -> Result<(), Box<dyn std::error::Error>> {
    let buf = minicbor_serde::to_vec(&command)?;
    stream.write(&buf)?;

    if command == StreamCommand::Flush {
        println!("Flushing {:?}", buf);
    }
    // sleep for a bit
//    std::thread::sleep(std::time::Duration::from_millis(15));
    stream.flush()?;
    Ok(())
}