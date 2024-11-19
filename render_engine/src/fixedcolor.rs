
pub type T = fixed::FixedI32<fixed::types::extra::U24>;

#[derive(Clone, Copy, Default)]
pub struct FixedColor {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
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
