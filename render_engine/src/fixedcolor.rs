#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type T = fixed::FixedI32<fixed::types::extra::U24>;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FixedColor {
    #[cfg_attr(feature = "serde",serde(with = "serialize_as_i32"))]
    pub r: T,
    #[cfg_attr(feature = "serde",serde(with = "serialize_as_i32"))]
    pub g: T,
    #[cfg_attr(feature = "serde",serde(with = "serialize_as_i32"))]
    pub b: T,
    #[cfg_attr(feature = "serde",serde(with = "serialize_as_i32"))]
    pub a: T,
}


#[cfg(feature = "serde")]
mod serialize_as_i32 {
    use super::T;
    use serde::{self, Serializer, Deserializer, Deserialize};

    pub fn serialize<S>(v: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(v.to_bits())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = i32::deserialize(deserializer)?;
        Ok(T::from_bits(bits))
    }
}


impl FixedColor {
    pub const WHITE: Self = Self {
        r: T::ONE,
        g: T::ONE,
        b: T::ONE,
        a: T::ONE,
    };


    
    pub fn rgb(r: T, g: T, b: T) -> Self {
        Self { r, g, b, a: T::ONE }
    }

    pub fn as_rgb8(&self) -> (u8, u8, u8) {
        fn as_u8(value: T) -> u8 {
            if value >= T::ONE {
                return 255;
            } else if value < T::ZERO {
                return 0;
            }
            (value * 255).to_num::<u8>()
        }
        (
            as_u8(self.r),
            as_u8(self.g),
            as_u8(self.b),
        )
    }

    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: T::from_num(r) / 255,
            g: T::from_num(g) / 255,
            b: T::from_num(b) / 255,
            a: T::ONE,
        }
    }

    pub fn scale(&self, scale: T) -> Self {
        Self {
            r: self.r.saturating_mul(scale),
            g: self.g.saturating_mul(scale),
            b: self.b.saturating_mul(scale),
            a: self.a,
        }
    }
    pub fn saturating_add(&self, other: Self) -> Self {
        Self {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
            a: self.a,
        }
    }
}


#[cfg(test)]
mod tests {
    use serde_binary::binary_stream::Endian;

    use super::*;

    #[test]
    fn test_fixedcolor() {
        let c = FixedColor::rgb(T::ONE, T::ONE, T::ONE);
        assert_eq!(c.as_rgb8(), (255, 255, 255));
    }

    // run this test if the serde feature is enabled

    #[cfg(feature = "serde")]
    #[test]
    fn test_fixedcolor_serde() {
        let c = FixedColor::rgb(T::ONE, T::ONE, T::ONE);
        let b = serde_binary::to_vec(&c, Endian::Big).unwrap();
        let c2: FixedColor = serde_binary::from_slice(&b, Endian::Big).unwrap();
        assert_eq!(c, c2);


        let c3 = FixedColor::from_rgb8(123, 33, 77);
        let b = serde_binary::to_vec(&c3, Endian::Big).unwrap();
        let c4 : FixedColor = serde_binary::from_slice(&b, Endian::Big).unwrap();
        assert_eq!(c3, c4);
    }
}