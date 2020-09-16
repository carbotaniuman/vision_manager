use opencv::prelude::*; 
use opencv::types::*;

fn main() {
    let mut a = VectorOfbool::new();
    a.push(true);
    a.push(true);
    a.push(true);
    a.push(true);
    a.push(true);
    a.push(true);
    a.push(true);
    a.push(true);
    a.push(true);
    a.push(true);
    a.push(true);
    println!("{:?}", a);
}