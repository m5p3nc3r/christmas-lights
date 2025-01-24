#![no_std]

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Animation {
    None,
    Snow,
    Sparkle,
    Rainbow
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Command {
    // Clear the display to a specific colour
    Clear(u8, u8, u8),
    Flush,
    Animate(Animation),
//    SetFrom(Buffer)
}
