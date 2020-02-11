extern crate gl;
extern crate half;
extern crate nalgebra;
extern crate sdl2;
extern crate vec_2_10_10_10;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate render_gl_derive;

mod debug;
pub mod render_gl;
pub mod resources;
mod triangle;
mod grid;

use triangle::Uniform;
use failure::err_msg;
use nalgebra as na;
use crate::resources::Resources;
use std::path::Path;
use std::time::{Duration, SystemTime};

const WINDOW_WIDTH: u32 = 1600;
const WINDOW_HEIGHT: u32 = 900;



fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }
}

fn run() -> Result<(), failure::Error> {

    // Window and OpenGL init
    let res = Resources::from_relative_exe_path(Path::new("res")).unwrap();
    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    let window = video_subsystem
        .window("Playground", WINDOW_WIDTH, WINDOW_HEIGHT)
        .opengl()
        .build()?;
    let _gl_context = window.gl_create_context().map_err(err_msg)?;
    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });
    let mut viewport = render_gl::Viewport::for_window(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
    

    // Setting up screen
    let color_buffer = render_gl::ColorBuffer::from_color(na::Vector3::new(0.0, 0.0, 0.0));
    let triangle = triangle::Triangle::new(&res, &gl)?;
    viewport.set_used(&gl);
    color_buffer.set_used(&gl);


    // Setting up for the loop
    let time0 = SystemTime::now();


    // main loop
    let mut event_pump = sdl.event_pump().map_err(err_msg)?;
    'main: loop {

        // Input
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    viewport.update_size(w, h);
                    viewport.set_used(&gl);
                }
                _ => {}
            }
        }

        // updating
        let time_since_start: f32 = (time0.elapsed()?.as_micros() as f32) / 1_000_000f32;

        // uniforms
        triangle.set_uniform("iResolution".to_string(), Uniform::Float2(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32), &gl);
        triangle.set_uniform("iTime".to_string(), Uniform::Float(time_since_start), &gl);

        // rendering
        color_buffer.clear(&gl);
        triangle.render(&gl);

        window.gl_swap_window();

    }

    Ok(())
}