
extern crate shader_macro;
extern crate cgmath;
use std::ffi::{ CString};

use cgmath::*;

shader_macro::shader_program!(
    Simple{
        uniform int CNT; 
        uniform mat4 MVP;
    }
);

fn main() {
    let mut s = Simple::new();
    if false {
        let program = 0;
        s.setup(&program);
        s.set_CNT(123);
        s.set_MVP(Matrix4::identity());
    }
    println!("{:?}",s);
}
