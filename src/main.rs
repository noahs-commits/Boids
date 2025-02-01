use crate::render::*;

pub mod lib;
pub mod render;
pub mod vertex;
pub mod boid;
//use tutorial1_window::run;

fn main() {
    pollster::block_on(render::run());
}