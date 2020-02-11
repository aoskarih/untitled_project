use failure;
use gl;
use std::ffi::c_void;
use crate::render_gl::{self, buffer, data};
use crate::resources::Resources;


pub enum Uniform {
    Int(i32),
    Int2(i32, i32),
    Int3(i32, i32, i32),
    Int4(i32, i32, i32, i32),
    Float(f32),
    Float2(f32, f32),
    Float3(f32, f32, f32),
    Float4(f32, f32, f32, f32),
}

impl Uniform {
    fn set_uniform(&self, location: gl::types::GLint, gl: &gl::Gl) {
        match self {
            Uniform::Int(x) => {
                unsafe {
                    gl.Uniform1i(location, *x);
                }
            },
            Uniform::Int2(x, y) => {
                unsafe {
                    gl.Uniform2i(location, *x, *y);
                }
            },
            Uniform::Int3(x, y, z) => {
                unsafe {
                    gl.Uniform3i(location, *x, *y, *z);
                }
            },
            Uniform::Int4(x, y, z, w) => {
                unsafe {
                    gl.Uniform4i(location, *x, *y, *z, *w);
                }
            },
            Uniform::Float(x) => {
                unsafe {
                    gl.Uniform1f(location, *x);
                }
            },
            Uniform::Float2(x, y) => {
                unsafe {
                    gl.Uniform2f(location, *x, *y);
                }
            },
            Uniform::Float3(x, y, z) => {
                unsafe {
                    gl.Uniform3f(location, *x, *y, *z);
                }
            },
            Uniform::Float4(x, y, z, w) => {
                unsafe {
                    gl.Uniform4f(location, *x, *y, *z, *w);
                }
            },
            _ => {}
        }

    }
}

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: data::f32_f32_f32,
    #[location = "1"]
    clr: data::u2_u10_u10_u10_rev_float,
}

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Indice {
    #[location = "0"]
    pos: data::i32_i32_i32,
}

pub struct Triangle {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer, // _ to disable warning about not used vbo
    vao: buffer::VertexArray,
}

impl Triangle {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Triangle, failure::Error> {
        // set up shader program

        let program = render_gl::Program::from_res(gl, res, "shaders/triangle")?;

        // set up vertex buffer object

        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: (1.0, -1.0, 0.0).into(),
                clr: (1.0, 0.0, 0.0, 1.0).into(),
            }, // bottom right
            Vertex {
                pos: (-1.0, -1.0, 0.0).into(),
                clr: (0.0, 1.0, 0.0, 1.0).into(),
            }, // bottom left
            Vertex {
                pos: (-1.0, 1.0, 0.0).into(),
                clr: (0.0, 0.0, 1.0, 1.0).into(),
            }, // top left
            Vertex {
                pos: (1.0, 1.0, 0.0).into(),
                clr: (0.0, 1.0, 0.0, 1.0).into(),
            }, // top right
        ];

        let vbo = buffer::ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        /*let ebo = buffer::ElementArrayBuffer::new(gl);
        ebo.bind();
        ebo.static_draw_data(&indicies);
        ebo.unbind();*/

        // set up vertex array object

        let vao = buffer::VertexArray::new(gl);

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(gl);
        vbo.unbind();
        vao.unbind();

        Ok(Triangle {
            program,
            _vbo: vbo,
            vao,
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();
        
        let mut ind: [i32; 6] = [0, 1, 3, 1, 2, 3]; 

        unsafe {
            let i_ptr: *mut c_void = &mut ind as *mut _ as *mut c_void;
            gl.DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                i_ptr
            );
            /*gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices to be rendered
            );*/
        }
    }

    pub fn set_uniform(&self, name: String, value: Uniform, gl: &gl::Gl) {
        let n_ptr = name.as_ptr();
        self.program.set_used();
        unsafe {
            let loc = gl.GetUniformLocation(self.program.id(), n_ptr as *const i8);
            value.set_uniform(loc, gl);
        }
    }
}