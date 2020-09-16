#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(const_generics)]

mod manager;
mod pipeline;

use crate::manager::*;
use crate::pipeline::*;
use modules::opencv::camera_input::CameraInput;
use modules::opencv::debug_display::DebugDisplay;
use modules::opencv::hsv_filter::HsvFilter;
use vision_traits::*;

fn main() {
    let mut m = Manager::new();

    m.register::<CameraInput>();
    m.register::<HsvFilter>();
    m.register::<DebugDisplay>();

    let cam_input = m.make_node("CameraInput", r#"{"num":0,"brightness":5,"exposure":5,"saturation":5,"contrast":5}"#);
    let hsv_filter = m.make_node("HsvFilter", r#"{"hue":{"min": 0, "max": 50},"saturation":{"min": 0, "max": 50},"value":{"min": 0, "max": 50}}"#);
    let deb_display = m.make_node("DebugDisplay", r#"{"name":"window"}"#);
    
    let mut a = Pipeline::new();
    let cam_id = a.add_node(cam_input);
    let hsv_id = a.add_node(hsv_filter);
    let deb_id = a.add_node(deb_display);

    println!("cam {:?}", cam_id);
    println!("deb {:?}", deb_id);

    a.connect(
        hsv_id,
        "val".to_string(),
        Connection {
            output_uuid: cam_id,
            output_name: "val".to_string(),
        },
    );

    a.connect(
        deb_id,
        "val".to_string(),
        Connection {
            output_uuid: hsv_id,
            output_name: "val".to_string(),
        },
    );
    loop {
        a.run_iteration();
    }
}
