#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type T = f32;//fixed::FixedI32<fixed::types::extra::U24>;
const ZERO:T = 0.0;
const ONE:T = 1.0;
const MAX_U8: T = 255.0;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FixedColor {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl FixedColor {
    pub const WHITE: Self = Self {
        r: ONE,
        g: ONE,
        b: ONE,
        a: ONE,
    };
    
    pub fn rgb(r: T, g: T, b: T) -> Self {
        Self { r, g, b, a: ONE }
    }

    pub fn as_rgb8(&self) -> (u8, u8, u8) {
        fn as_u8(value: T) -> u8 {
            if value >= ONE {
                return MAX_U8 as u8;
            } else if value < ZERO {
                return 0;
            }
            (value * 255.0) as u8//.to_num::<u8>()
        }
        (
            as_u8(self.r),
            as_u8(self.g),
            as_u8(self.b),
        )
    }

    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {

        Self {
            r: (r as f32) / MAX_U8,
            g: (g as f32) / MAX_U8,
            b: (b as f32) / MAX_U8,
            a: ONE,
        }
    }

    pub fn scale(&self, scale: T) -> Self {
        Self {
            r: (self.r * scale).clamp(ZERO, ONE),
            g: (self.g * scale).clamp(ZERO, ONE),
            b: (self.b * scale).clamp(ZERO, ONE),
            a: self.a,
        }
    }
    pub fn saturating_add(&self, other: Self) -> Self {
        Self {
            r: (self.r + other.r).clamp(ZERO, ONE),
            g: (self.g + other.g).clamp(ZERO, ONE),
            b: (self.b + other.b).clamp(ZERO, ONE),
            a: self.a,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixedcolor() {
        let c = FixedColor::rgb(1.0, 1.0, 1.0);
        assert_eq!(c.as_rgb8(), (255, 255, 255));
    }

    // run this test if the serde feature is enabled

    #[cfg(feature = "serde")]
    mod serde_tests {
        use super::*;
        use ciborium::{de::from_reader, ser::into_writer};
        #[test]
        fn test_fixedcolor_serde() {

            let mut buffer = [0u8; 32];

            let c = FixedColor::rgb(1.0, 1.0, 1.0);
            let _ = into_writer(&c, &mut buffer[..]).unwrap();

            let c2: FixedColor = from_reader(&buffer[..]).unwrap();
            assert_eq!(c, c2);


            let c3 = FixedColor::from_rgb8(123, 33, 77);
            let _ = into_writer(&c3, &mut buffer[..]).unwrap();
            let c4 : FixedColor = from_reader(&buffer[..]).unwrap();
            assert_eq!(c3, c4);
        }
    }
}