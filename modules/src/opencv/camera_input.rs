
use opencv::prelude::*;
use opencv::videoio::*;
use vision_traits::{Configurable, DynErrResult, Node, output::OutputSingular};

#[derive(Configurable)]
pub struct CameraInputS {
    num: i32,
    brightness: i32,
    contrast: i32,
    saturation: i32,
    exposure: i32,
}

pub struct CameraInput {
    cap: VideoCapture,
}

impl Node for CameraInput {
    const NAME: &'static str = "CameraInput";

    type S = CameraInputS;
    type I<'a> = ();
    type O = OutputSingular<Mat>;

    fn make(settings: Self::S) -> DynErrResult<Self> {
        let mut cap = VideoCapture::new(settings.num, CAP_ANY)?;
        cap.set(CAP_PROP_BRIGHTNESS, settings.brightness as f64)?;
        cap.set(CAP_PROP_CONTRAST, settings.contrast as f64)?;
        cap.set(CAP_PROP_SATURATION, settings.saturation as f64)?;
        cap.set(CAP_PROP_EXPOSURE, settings.exposure as f64)?;
        Ok(Self { cap })
    }

    fn process(&mut self, _: Self::I<'_>) -> DynErrResult<Self::O> {
        let mut mat = Mat::default()?;
        self.cap.read(&mut mat)?;
        Ok(mat.into())
    }
}