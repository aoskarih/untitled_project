use failure;
use gl;
use crate::render_gl::{self, buffer, data};
use crate::resources::Resources;

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: data::f32_f32_f32,
    #[location = "1"]
    clr: data::f32_,
}

pub struct Grid {
    program: render_gl::Program,
    vbo: buffer::ArrayBuffer, 
    vao: buffer::VertexArray,
    width: u32,
    height: u32,
}

impl Grid {
    pub fn new(res: &Resources, gl: &gl::Gl, w: u32, h: u32) -> Result<Grid, failure::Error> {
        
        // set up shader program

        let program = render_gl::Program::from_res(gl, res, "shaders/grid")?;

        // set up vertex buffer object

        let mut vertices: Vec<Vertex> = vec![];

        for x in 0..(w-1) {
            for y in 0..(h-1) {
                vertices.push(Vertex {
                    pos: ((x as f32)/w as f32, (y as f32)/h as f32, 0.0).into(),
                    clr: (0.0).into()
                });
                vertices.push(Vertex {
                    pos: (((x+1) as f32)/w as f32, (y as f32)/h as f32, 0.0).into(),
                    clr: (0.0).into()
                });
                vertices.push(Vertex {
                    pos: ((x as f32)/w as f32, ((y+1) as f32)/h as f32, 0.0).into(),
                    clr: (0.0).into()
                });

                vertices.push(Vertex {
                    pos: (((x+1) as f32)/w as f32, (y as f32)/h as f32, 0.0).into(),
                    clr: (0.0).into()
                });
                vertices.push(Vertex {
                    pos: ((x as f32)/w as f32, ((y+1) as f32)/h as f32, 0.0).into(),
                    clr: (0.0).into()
                });
                vertices.push(Vertex {
                    pos: (((x+1) as f32)/w as f32, ((y+1) as f32)/h as f32, 0.0).into(),
                    clr: (0.0).into()
                });
            }
        }

        let vbo = buffer::ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        // set up vertex array object

        let vao = buffer::VertexArray::new(gl);

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(gl);
        vbo.unbind();
        vao.unbind();

        Ok(Grid {
            program,
            vbo: vbo,
            vao,
            width: w,
            height: h,
        })
    }

    pub fn update(&self, v_data: &[f32]) {
        
        let mut vertices: Vec<Vertex> = vec![];

        for x in 0..(self.width-1) {
            for y in 0..(self.height-1) {

                

                vertices.push(Vertex {
                    pos: (((2*x) as f32)/self.width as f32 - 1.0, ((2*y) as f32)/self.height as f32 - 1.0, 0.0).into(),
                    clr: (0.0).into()
                });
                vertices.push(Vertex {
                    pos: (((2*x+2) as f32)/self.width as f32 - 1.0, ((2*y) as f32)/self.height as f32 - 1.0, 0.0).into(),
                    clr: (0.0).into()
                });
                vertices.push(Vertex {
                    pos: (((2*x) as f32)/self.width as f32 - 1.0, ((2*y+2) as f32)/self.height as f32 - 1.0, 0.0).into(),
                    clr: (0.0).into()
                });

                vertices.push(Vertex {
                    pos: (((2*x+2) as f32)/self.width as f32 - 1.0, ((2*y+2) as f32)/self.height as f32 - 1.0, 0.0).into(),
                    clr: (0.0).into()
                });
                vertices.push(Vertex {
                    pos: (((2*x) as f32)/self.width as f32 - 1.0, ((2*y+2) as f32)/self.height as f32 - 1.0, 0.0).into(),
                    clr: (0.0).into()
                });
                vertices.push(Vertex {
                    pos: (((2*x+2) as f32)/self.width as f32 - 1.0, ((2*y+2) as f32)/self.height as f32 - 1.0, 0.0).into(),
                    clr: (0.0).into()
                });
            }
        }

        self.vbo.bind();
        self.vbo.static_draw_data(&vertices);
        self.vbo.unbind();

    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                ((self.width-1)*(self.height-1)*2*3) as i32,             // number of indices to be rendered
            );
        }
    }
}