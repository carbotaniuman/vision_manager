
use opencv::prelude::*;
use opencv::imgproc::*;
use opencv::core::*;
use vision_traits::{Configurable, DynErrResult, input::InputSingular, Node, output::OutputSingular, types::range::RangeU8};

#[derive(Configurable)]
pub struct HsvFilterS {
    hue: RangeU8<0, 180>,
    saturation: RangeU8<0, 255>,
    value: RangeU8<0, 255>,
}

pub struct HsvFilter {
    settings: HsvFilterS,
}

impl Node for HsvFilter {
    const NAME: &'static str = "HsvFilter";

    type S = HsvFilterS;
    type I<'a> = InputSingular<'a, Mat>;
    type O = OutputSingular<Mat>;

    fn make(settings: Self::S) -> DynErrResult<Self> {
        Ok(Self { settings })
    }

    fn process(&mut self, mat: Self::I<'_>) -> DynErrResult<Self::O> {
        let mut hsv = Mat::default()?;
        cvt_color(&(mat.val), &mut hsv, COLOR_BGR2HSV, 0)?;
        let mut filtered = Mat::default()?;
        in_range(&hsv,
            &Mat::from_slice(&[self.settings.hue.min, self.settings.saturation.min, self.settings.value.min])?,
            &Mat::from_slice(&[self.settings.hue.max, self.settings.saturation.max, self.settings.value.max])?,  
            &mut filtered)?;
        Ok(filtered.into())
    }
}