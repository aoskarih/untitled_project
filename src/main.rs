extern crate sdl2; 
extern crate rand;
extern crate libc;
extern crate time;

use sdl2::sys;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

use time::PreciseTime;
use std::time::Duration;

use rand::Rng;

// // Constants
// Rendering
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

const TEX_W: i32 = 320;
const TEX_H: i32 = 180;


// // Traits
pub trait Draw {
    fn draw(&self, tex: &mut [u8; (TEX_H*TEX_W*4) as usize], cam: &Camera);
    fn on_screen(&self, cam: &Camera) -> bool;
}

pub trait Move {
    fn move_amount(&mut self, dx: i32, dy: i32);
    fn move_to(&mut self, x: i32, y: i32);
}

// // "Objects"
pub struct Square {
    x: i32,
    y: i32,
    w: i32,
    h: i32
}

pub struct ColorSquare {
    sqr: Square,
    col: [u8; 4] // a, b, g, r
}

pub struct Sprite {
    sqr: Square,
    data: Vec<u8>
}

pub enum Texture {
    Square(ColorSquare),
    Sprite(Sprite)
}

pub struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

pub struct Wall {
    line: Line
}

pub struct Polygon {
    lines: Vec<Line>
}

pub enum EnvObject {
    Wall(Wall),
    Polygon(Polygon)
}

pub struct Environment {
    walls: Vec<EnvObject>
}

struct Agent {
    x: i32,
    y: i32,
    vx: f32,
    vy: f32,
    speed: f32,
    tex: Texture
}

pub struct Camera {
    x: i32,
    y: i32,
}

// Impl

impl Agent {
    fn move_agent_to(&mut self, x: i32, y: i32, walls: &Vec<Wall>) -> bool {
        let coll: bool = false;
        let movement = Line {x1: self.x, y1: self.y, x2: x, y2: y};

        for wall in walls.iter() {
            // check if movement is intersecting wall.
        }
        if coll {
            // move to wall and then bounce?
            self.x = x;
            self.y = y;
        } else {
            self.x = x;
            self.y = y;
        }
        self.tex.move_to(self.x, self.y);

        return coll;
    }
    fn move_agent_amount(&mut self, dx: i32, dy: i32, walls: &Vec<Wall>) -> bool {
        return self.move_agent_to(self.x + dx, self.y + dy, walls);
    }
}

impl Draw for ColorSquare {
    fn draw(&self, tex: &mut [u8; (TEX_H*TEX_W*4) as usize], cam: &Camera) {
        if !self.on_screen(cam) {
            return;
        }
        let sx = self.sqr.x - cam.x;
        let sy = self.sqr.y - cam.y;

        'y: for j in 0..self.sqr.h {
            if j + sy < 0 {
                continue 'y;
            } else if j + sy + 1 > TEX_H {
                break 'y;
            }
            'x: for i in 0..self.sqr.w {
                if i + sx < 0 {
                    continue 'x;
                } else if i + sx + 1 > TEX_W {
                    continue 'y;
                }
                for c in 0..4 {
                    tex[(((sy+j)*TEX_W + sx + i)*4 + c) as usize] = self.col[c as usize];
                }
            }
        }

    }

    fn on_screen(&self, cam: &Camera) -> bool {
        let sx = self.sqr.x - cam.x;
        let sy = self.sqr.y - cam.y;

        if sx + self.sqr.w < 0 || sx > TEX_W {
            return false;
        } else if sy + self.sqr.h < 0 || sy > TEX_H {
            return false;
        }

        return true;
    }
}

impl Move for ColorSquare {
    fn move_amount(&mut self, dx: i32, dy: i32) {
        self.sqr.x += dx;
        self.sqr.y += dy;
    }
    fn move_to(&mut self, x: i32, y: i32) {
        self.sqr.x = x;
        self.sqr.y = y;
    }
}

impl Move for Sprite {
    fn move_amount(&mut self, dx: i32, dy: i32) {
        self.sqr.x += dx;
        self.sqr.y += dy;
    }
    fn move_to(&mut self, x: i32, y: i32) {
        self.sqr.x = x;
        self.sqr.y = y;
    }
}

impl Draw for Texture {
    fn draw(&self, tex: &mut [u8; (TEX_H*TEX_W*4) as usize], cam: &Camera) {
        match self {
            Texture::Square(color_square) => {
                color_square.draw(tex, cam);
            },
            _ => ()
        }
    }

    fn on_screen(&self, cam: &Camera) -> bool {
        match self {
            Texture::Square(color_square) => {
                return color_square.on_screen(cam);
            },
            _ => true
        }
        
    }
}

impl Move for Texture {
    fn move_amount(&mut self, dx: i32, dy: i32) {
        match self {
            Texture::Square(color_square) => {
                color_square.move_amount(dx, dy);
            },
            Texture::Sprite(sprite) => {
                sprite.move_amount(dx, dy);
            },
            _ => ()
        }
    }
    fn move_to(&mut self, x: i32, y: i32) {
        match self {
            Texture::Square(color_square) => {
                color_square.move_to(x, y);
            },
            Texture::Sprite(sprite) => {
                sprite.move_to(x, y);
            },
            _ => ()
        }
    }
}


fn main() {

    // Rendering
    let ctx = sdl2::init().unwrap();
    let _video_ctx = ctx.video().unwrap();

    let title = "Amplitude";
    
    unsafe {
    
    let window = sys::SDL_CreateWindow(title.as_ptr() as *const libc::c_char, 536805376, 536805376, WIDTH as i32, HEIGHT as i32, 4);
    let renderer = sys::SDL_CreateRenderer(window, -1, 2);
    let texture = sys::SDL_CreateTexture(renderer, 373694468, 1, TEX_W, TEX_H);
    
    let mut tex_data = [0u8; (TEX_H*TEX_W*4) as usize];
    let NULL: *const sys::SDL_Rect = std::ptr::null();

    // Input
    let mut event_pump = ctx.event_pump().unwrap();
    
    // Time
    let start = PreciseTime::now();
    let mut timer = PreciseTime::now();
    let mut dt = 0;
    let mut t = 0;
    let mut frame = 0;

    // Game
    let test_sqr = ColorSquare {
        sqr: Square {
            x: 100,
            y: 100,
            w: 50,
            h: 20
        },
        col: [0, 0, 0, 255]
    };

    let mut player_sqr = ColorSquare {
        sqr: Square {
            x: 0,
            y: 0,
            w: 30,
            h: 30
        },
        col: [255, 250, 100, 55]
    };
    let mut camera = Camera {x: 0, y: 0};
    
    let mut player = Agent{
        x: 0,
        y: 0,
        vx: 0.0,
        vy: 0.0,
        speed: 0.3,
        tex: Texture::Square(player_sqr)
    };

    let walls: Vec<Wall> = Vec::new();

    'running: loop {
        
        // Input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                // Control input
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    player.vx = -player.speed;
                },
                Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                    if player.vx < 0.0 {
                        player.vx = 0.0;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                     player.vx = player.speed;
                },
                Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                    if player.vx > 0.0 {
                        player.vx = 0.0;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    player.vy = -player.speed;
                },
                Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                    if player.vy < 0.0 {
                        player.vy = 0.0;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    player.vy = player.speed;
                },
                Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                    if player.vy > 0.0 {
                        player.vy = 0.0;
                    }
                },
                _ => {}
            }
        }
        
        // Time
        dt = timer.to(PreciseTime::now()).num_milliseconds();
        timer = PreciseTime::now();
        t = start.to(timer).num_milliseconds();
        frame += 1;

        // Updating

        player.move_agent_amount((player.vx*dt as f32) as i32, (player.vy*dt as f32) as i32, &walls);

        camera.x = player.x - TEX_W/2 + 15;
        camera.y = player.y - TEX_H/2 + 15;

        for i in 0..TEX_W {
            for j in 0..TEX_H {
                let a = p_col(i+camera.x, j+camera.y, t as i32);
                for k in 0..4 {
                    //tex_data[((TEX_W*j + i)*4 + k) as usize] = a[k as usize];
                    tex_data[((TEX_W*j + i)*4 + k) as usize] = 0;
                }
            }
        }
        

        test_sqr.draw(&mut tex_data, &camera);
        
        player.tex.draw(&mut tex_data, &camera);

        // Rendering

        sys::SDL_SetRenderDrawColor(renderer, 100, 100, 100, 255);
        sys::SDL_RenderClear(renderer);

        sys::SDL_UpdateTexture(texture, NULL, tex_data.as_ptr() as *const std::ffi::c_void, TEX_W*4);

        let a = sys::SDL_RenderCopy(renderer, texture, NULL, NULL);
        sys::SDL_RenderPresent(renderer);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    }
    let end = PreciseTime::now();
    let seconds = start.to(end).num_seconds();
    println!("seconds: {}", seconds);
    println!("Frames per second: {}", frame/seconds);

    sys::SDL_DestroyRenderer(renderer);
    sys::SDL_DestroyWindow(window);
    sys::SDL_Quit();
    
    }
}


fn p_col(x: i32, y: i32, t: i32) -> [u8; 4] {
    let mut a = [255u8; 4];
    let r1 = ((x*x + y*y) as f32).sqrt();
    let th = (y as f32).atan2(x as f32);
    // let r2 = (((x-TEX_W)*(x-TEX_W) + y*y) as f32).sqrt();
    // let r3 = ((x*x + (y-TEX_H)*(y-TEX_H)) as f32).sqrt();
    // let ar = [r1, r2, r3];
    for i in 0..3 {
        a[1+i as usize] = (((r1*0.4).sin()*(10.0*(th+((t*i) as f32)/2000.0)).sin() + 1.0)*128.0) as u8;
    }
    return a;
}
