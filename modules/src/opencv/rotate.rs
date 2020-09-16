use std::collections::HashMap;
use vision_traits::schema::SettingType;
use std::error::Error;
use opencv::prelude::*;
use opencv::core::*;
use vision_traits::{Configurable, DynErrResult, editable::Editable, input::InputSingular, Node, output::OutputSingular};
use vision_traits::json::{array, JsonValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotateType {
    Rotate90Clockwise,
    Rotate180,
    Rotate90Counterclockwise,
}

impl Editable for RotateType {
    fn schema() -> SettingType {
        let mut map = HashMap::new();

        map.insert("variants".to_owned(), array!("Rotate 90 Degreees Clockwise, Rotate 180 Degreees, Rotate 90 Degreees Counterclockwise"));

        SettingType {
            name: "RotateType".to_owned(),
            params: map,
        }
    }
    fn deserialize(_input: &JsonValue) -> Result<Self, Box<dyn Error>> {
        Ok(Self::Rotate90Clockwise)
    }
}

#[derive(Configurable)]
pub struct RotateS {
    mode: RotateType,
}

pub struct Rotate {
    settings: RotateS,
}

impl Node for Rotate {
    const NAME: &'static str = "Rotate";

    type S = RotateS;
    type I<'a> = InputSingular<'a, Mat>;
    type O = OutputSingular<Mat>;

    fn make(settings: Self::S) -> DynErrResult<Self> {
        Ok(Self { settings })
    }

    fn process(&mut self, mat: Self::I<'_>) -> DynErrResult<Self::O> {
        let mut out = Mat::default()?;
        rotate(mat.val, &mut out, self.settings.mode as i32)?;
        Ok(out.into())
    }
}