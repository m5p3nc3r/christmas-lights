#![no_std]

use serde::{Serialize, Deserialize};
//use render_engine::RenderBuffer;

//pub type SizedRenderBuffer = RenderBuffer<120, 5, 24>;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Animation {
    None,
    Snow,
    Sparkle,
    Rainbow
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Command {
    // Clear the display to a specific colour
    Clear(u8, u8, u8),
    Flush,
    Animate(Animation),
    SetPixel(u8, u8, u8, u8, u8), // x, y, r, g, b
//    SetBuffer(RenderBuffer<S, X, Y>) // x, y, buffer
}
