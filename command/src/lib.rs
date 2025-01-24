#![no_std]

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Animation {
    Snow,
    Sparkle,
    Rainbow
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Command {
    // Clear the display to a specific colour
    Clear(u8, u8, u8),
    Animate(Animation),
//    SetFrom(Buffer)
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
