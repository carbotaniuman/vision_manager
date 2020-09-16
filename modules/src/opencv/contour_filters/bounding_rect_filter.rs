
use opencv::prelude::*;
use opencv::imgproc::*;
use opencv::core::*;
use opencv::types::*;
use vision_traits::{Configurable, DynErrResult, input::InputSingular, Node, output::OutputSingular, types::range::RangeU8};

#[derive(Configurable)]
pub struct BoundingRectFilterS {
    hue: RangeU8<0, 180>,
    saturation: RangeU8<0, 255>,
    value: RangeU8<0, 255>,
}

pub struct BoundingRectFilter {
    settings: BoundingRectFilterS,
}

impl Node for BoundingRectFilter {
    const NAME: &'static str = "BoundingRectFilter";

    type S = BoundingRectFilterS;
    type I<'a> = InputSingular<'a, VectorOfVectorOfPoint>;
    type O = OutputSingular<VectorOfVectorOfPoint>;

    fn make(settings: Self::S) -> DynErrResult<Self> {
        Ok(Self { settings })
    }

    fn process(&mut self, contours: Self::I<'_>) -> DynErrResult<Self::O> {
        let mut contours = contours.val.iter().filter(|n| n + 5 > 44);

        Ok(contours.into())
    }
}