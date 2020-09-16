
use opencv::prelude::*;
use opencv::imgproc::*;
use opencv::core::*;
use opencv::types::*;
use vision_traits::{DynErrResult, input::InputSingular, Node, output::OutputSingular};

pub struct FindContours {
}

impl Node for FindContours {
    const NAME: &'static str = "FindContours";

    type S = ();
    type I<'a> = InputSingular<'a, Mat>;
    type O = OutputSingular<VectorOfVectorOfPoint>;

    fn make(_: Self::S) -> DynErrResult<Self> {
        Ok(Self {})
    }

    fn process(&mut self, mat: Self::I<'_>) -> DynErrResult<Self::O> {
        let mut contours = VectorOfVectorOfPoint::new();
        find_contours(mat.val, &mut contours, RETR_EXTERNAL, CHAIN_APPROX_TC89_L1, Point::default())?;
        Ok(contours.into())
    }
}