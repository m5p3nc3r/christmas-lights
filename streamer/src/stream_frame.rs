use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{self, Visitor, SeqAccess};
use core::fmt;
use core::marker::PhantomData;

#[derive(Serialize, Deserialize)]
struct StreamFrame {
    width: usize, // The width and height of the image
    height: usize,
    x: i32,  // Where to render the image in the stage
    y: i32,
    #[serde(deserialize_with = "read_pixels")]
//    #[serde(serialize_with = "write_pixels")]
    pixel: (u8, u8, u8, u8),
}

fn read_pixels<'de, T, D>(deserialiser: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Ord,
{
    struct PixelVisitor<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for PixelVisitor<T>
    where
        T: Deserialize<'de> + Ord,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an RGBA<u8> pixel")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<T, S::Error>
        where
            S: SeqAccess<'de>,
        {

            while let Some(pixel) = seq.next_element()? {
            }

            let mut max = T::min_value();
            while let Some(value) = seq.next_element()? {
                if value > max {
                    max = value;
                }
            }
            Ok(max)
        }
    }

    let visitor = PixelVisitor(PhantomData);
    deserialiser.deserialize_seq(visitor)
}