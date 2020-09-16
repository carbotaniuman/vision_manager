use opencv::prelude::*;
use opencv::highgui::*;
use vision_traits::{Configurable, DynErrResult, input::InputSingular, Node};

#[derive(Configurable)]
pub struct DebugDisplayS {
    name: String,
}

pub struct DebugDisplay {
    settings: DebugDisplayS,
}

impl Node for DebugDisplay {
    const NAME: &'static str = "DebugDisplay";

    type S = DebugDisplayS;
    type I<'a> = InputSingular<'a, Mat>;
    type O = ();

    fn make(settings: Self::S) -> DynErrResult<Self> {
        Ok(Self { settings })
    }

    fn process(&mut self, mat: Self::I<'_>) -> DynErrResult<Self::O> {
        imshow(&self.settings.name, &(mat.val))?;
        wait_key(1)?;
        Ok(())
    }
}