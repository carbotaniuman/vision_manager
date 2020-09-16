use std::collections::HashMap;
use vision_traits::schema::SettingType;
use std::error::Error;
use opencv::prelude::*;
use opencv::core::*;
use opencv::imgproc::*;
use vision_traits::{Configurable, DynErrResult, editable::Editable, input::InputSingular, Node, output::OutputSingular};
use vision_traits::json::{array, JsonValue};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlurType {
    Box,
    Gaussian,
    Median,
}

impl Editable for BlurType {
    fn schema() -> SettingType {
        let mut map = HashMap::new();

        map.insert("variants".to_owned(), array!("Box, Gaussian, Median"));

        SettingType {
            name: "BlurType".to_owned(),
            params: map,
        }
    }
    fn deserialize(_input: &JsonValue) -> Result<Self, Box<dyn Error>> {
        Ok(Self::Box)
    }
}

#[derive(Configurable)]
pub struct BlurS {
    blur_size: i32,
    blur_type: BlurType,
}

pub struct Blur {
    settings: BlurS,
}

impl Node for Blur {
    const NAME: &'static str = "Blur";

    type S = BlurS;
    type I<'a> = InputSingular<'a, Mat>;
    type O = OutputSingular<Mat>;

    fn make(settings: Self::S) -> DynErrResult<Self> {
        Ok(Self { settings })
    }

    fn process(&mut self, mat: Self::I<'_>) -> DynErrResult<Self::O> {
        let mut out = Mat::default()?;
        match self.settings.blur_type {
            BlurType::Box => blur(mat.val, &mut out, Size::new(self.settings.blur_size, self.settings.blur_size), Point::new(-1, -1), BORDER_DEFAULT)?,
            BlurType::Gaussian => gaussian_blur(mat.val, &mut out, Size::new(self.settings.blur_size, self.settings.blur_size), 0f64, 0f64, BORDER_DEFAULT)?,
            BlurType::Median => median_blur(mat.val, &mut out, self.settings.blur_size)?,
        };
        Ok(out.into())
    }

}