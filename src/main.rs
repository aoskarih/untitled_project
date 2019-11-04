extern crate sdl2; 
extern crate rand;
extern crate libc;
extern crate time;

use sdl2::sys;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

use time::PreciseTime;
use std::time::Duration;

use std::cmp;
use rand::Rng;

// // Misc
/*
let a = Point {x: movement.x2 - movement.x1, y: movement.y2 - movement.y1};
'wall_check: for wall in walls.iter() {
    let b = Point {x: wall.line.x1 - wall.line.x2, y: wall.line.y1 - wall.line.y2};
    let c = Point {x: movement.x1 - wall.line.x1, y: movement.y1 - wall.line.y1};
    
    let al_nu = b.y*c.x-b.x*c.y;
    let al_de = a.y*b.x-a.x*b.y;
    let be_nu = c.y*a.x-c.x*a.y;

    if al_de == 0 {
        continue 'wall_check;
    }
    let al = al_nu as f32/al_de as f32;
    if al < 0.0 || al > 1.0 {
        continue 'wall_check;
    }
    let be = be_nu as f32/al_de as f32;
    if be < 0.0 || be > 1.0 {
        continue 'wall_check;
    }
    let p_col = Point {x: movement.x1 + (al*a.x as f32) as i32, y: movement.y1 + (al*a.y as f32) as i32};
}
*/

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
    Sprite(Sprite),
    Line(ColorLine)
}

pub struct Point {
    x: i32,
    y: i32
}

pub struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

pub struct ColorLine {
    line: Line,
    col: [u8; 4] // a, b, g, r
}

pub struct ColorPoint {
    point: Point,
    col: [u8; 4] // a, b, g, r
}

pub struct Wall {
    line: Line
}

pub struct Polygon {
    lines: Vec<Line>
}

pub struct Mesh {
    dx: i32,
    dy: i32,
    r: i32
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
    tex: Texture,
    mesh: Mesh
}

pub struct Camera {
    x: i32,
    y: i32,
}

// Impl

impl Agent {
    
    fn move_agent_to(&mut self, x: i32, y: i32, walls: &Vec<Wall>) -> bool {
        let mut coll: bool = false;
        let movement = Line {x1: self.x + self.mesh.dx, y1: self.y + self.mesh.dy, x2: x + self.mesh.dx, y2: y + self.mesh.dy};
        let l = movement.lenght();
        let mut p_end = Point {x: movement.x2, y: movement.y2};
        if l < self.mesh.r as f32 {
            'wall_check: for wall in walls.iter() {
                let d = wall.line.distance_to_point(&p_end);
                if d < self.mesh.r as f32 {
                    let mut n = wall.line.normal_vector();
                    println!("d: {}, n: {} {}", d, n[0], n[1]);
                    let mn = movement.direction_vector();
                    if mn[0] * n[0] + mn[1] * n[1] > 0.0 {
                        n = [-n[0],-n[1]];
                    } 
                    p_end.x += ((self.mesh.r as f32 - d)*n[0]) as i32;
                    p_end.y += ((self.mesh.r as f32 - d)*n[1]) as i32;
                    coll = true;
                } else {
                    continue 'wall_check;
                }
            }
        }

        self.x = p_end.x - self.mesh.dx;
        self.y = p_end.y - self.mesh.dy;
        self.tex.move_to(self.x, self.y);

        return coll;
    }

    fn move_agent_amount(&mut self, dx: i32, dy: i32, walls: &Vec<Wall>) -> bool {
        return self.move_agent_to(self.x + dx, self.y + dy, walls);
    }

}

impl Point {
    fn on_screen(&self, cam: &Camera) -> bool {
        if self.x > cam.x && self.x < TEX_W + cam.x {
            if self.y > cam.y && self.y < TEX_H + cam.y { 
                return true;
            }
        }
        return false;
    }
}

impl Line {
    fn lenght_vec(&self) -> [i32; 2] {
        return [(self.x2-self.x1).abs(), (self.y2-self.y1).abs()];
    }
    
    fn lenght(&self) -> f32 {
        return (((self.x2-self.x1)*(self.x2-self.x1) + (self.y2-self.y1)*(self.y2-self.y1)) as f32).sqrt();
    }

    fn distance_to_point(&self, p: &Point) -> f32{
        let ax = self.x2-self.x1;
        let ay = self.y2-self.y1;

        let norm = ax*ax + ay*ay;

        let mut u = ((p.x - self.x1) * ax + (p.y - self.y1) * ay) as f32 / norm as f32;

        if u > 1.0 {
            u = 1.0;
        } else if u < 0.0 {
            u = 0.0;
        }

        let x = self.x1 as f32 + u * ax as f32;
        let y = self.y1 as f32 + u * ay as f32;

        let dx = x - p.x as f32;
        let dy = y - p.y as f32;

        let dist = (dx * dx + dy * dy).sqrt();

        return dist;
    }

    fn normal_vector(&self) -> [f32; 2] {
        if self.x1 == self.x2 {
            return [-1.0, 0.0];
        } else if self.y1 == self.y2 {
            return [0.0, 1.0];
        }
        let k = (self.y2 - self.y1) as f32 / (self.x2 - self.x1) as f32;
        let n = (k*k + 1.0).sqrt();
        return [-k/n, 1.0/n];
    }

    fn direction_vector(&self) -> [f32; 2] {
        let n = self.lenght();
        let x = (self.x2-self.x1) as f32 / n;
        let y = (self.y2-self.y1) as f32 / n;
        return [x, y];
    }

}

impl Draw for ColorPoint {
    fn draw(&self, tex: &mut [u8; (TEX_H*TEX_W*4) as usize], cam: &Camera) { 
        if !self.on_screen(cam) {
            return;
        }
        let sx = self.point.x - cam.x;
        let sy = self.point.y - cam.y;
        for c in 0..4 {
            tex[((sy*TEX_W + sx)*4 + c) as usize] = self.col[c as usize];
        }
    }

    fn on_screen(&self, cam: &Camera) -> bool {
        return self.point.on_screen(cam);
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

impl Draw for ColorLine {
    fn draw(&self, tex: &mut [u8; (TEX_H*TEX_W*4) as usize], cam: &Camera) { 
        if !self.on_screen(cam) {
            return;
        }
        let lx = self.line.x2-self.line.x1;
        let ly = self.line.y2-self.line.y1;

        if lx.abs() > ly.abs() {
            let dy = ly as f32 / lx as f32;
            for x in 0..lx {
                let sy = (self.line.y1 as f32 + dy * x as f32) as i32 - cam.y;
                let sx = self.line.x1 + x - cam.x;

                if sx < 0 || sx + 1 > TEX_W || sy < 0 || sy + 1 > TEX_H {
                    continue;
                }

                for c in 0..4 {
                    tex[(((sy as i32)*TEX_W + sx)*4 + c) as usize] = self.col[c as usize];
                }
            }
        } else {
            let dx = lx as f32 / ly as f32;
            for y in 0..ly {
                let sx = (self.line.x1 as f32 + dx * y as f32) as i32 - cam.x;
                let sy = self.line.y1 + y - cam.y;

                if sx < 0 || sx + 1 > TEX_W || sy < 0 || sy + 1 > TEX_H {
                    continue;
                }

                for c in 0..4 {
                    tex[(((sy as i32)*TEX_W + sx)*4 + c) as usize] = self.col[c as usize];
                }
            }
        }

    }

    fn on_screen(&self, cam: &Camera) -> bool {
        let l = self.line.lenght_vec();
        if l[0] < TEX_W && l[1] < TEX_H {
            let p1 = Point {x: self.line.x1, y: self.line.y1};
            let p2 = Point {x: self.line.x2, y: self.line.y2};
            if p1.on_screen(cam) || p2.on_screen(cam) {
                return true;
            }
        } else {
            let mut p: Vec<Point> = Vec::new();
            for q in 0..11 {
                p.push(Point {x: ((10-q)*self.line.x1 + q*self.line.x2)/10, y: ((10-q)*self.line.y1 + q*self.line.y2)/10});
            }
            for point in p.iter() {
                if point.on_screen(cam) {
                    return true;
                }
            }
        }
        return false;
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

impl Move for ColorLine {
    fn move_amount(&mut self, dx: i32, dy: i32) {
        self.move_to(self.line.x1 + dx, self.line.y1 + dy);
    }
    fn move_to(&mut self, x: i32, y: i32) {
        self.line.x2 = self.line.x2 - self.line.x1 + x;
        self.line.y2 = self.line.y2 - self.line.y1 + y;
        self.line.x1 = x;
        self.line.y1 = y;
    }
}

impl Draw for Texture {
    fn draw(&self, tex: &mut [u8; (TEX_H*TEX_W*4) as usize], cam: &Camera) {
        match self {
            Texture::Square(color_square) => {
                color_square.draw(tex, cam);
            },
            Texture::Line(color_line) => {
                color_line.draw(tex, cam);
            },
            _ => ()
        }
    }

    fn on_screen(&self, cam: &Camera) -> bool {
        match self {
            Texture::Square(color_square) => {
                return color_square.on_screen(cam);
            },
            Texture::Line(color_line) => {
                return color_line.on_screen(cam);
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
            Texture::Line(line) => {
                line.move_amount(dx, dy);
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
            Texture::Line(line) => {
                line.move_to(x, y);
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

    let c_line = ColorLine {
        line: Line {
            x1: 200,
            y1: 200,
            x2: 800,
            y2: 1000
        },
        col: [255, 50, 250, 100]
    };

    let test_wall = Wall {
        line: Line {
            x1: 200,
            y1: 200,
            x2: 800,
            y2: 1000
        }
    };

    let mut camera = Camera {x: 0, y: 0};
    
    let mut player = Agent{
        x: 0,
        y: 0,
        vx: 0.0,
        vy: 0.0,
        speed: 0.3,
        tex: Texture::Square(player_sqr),
        mesh: Mesh {dx: 15, dy: 15, r: 15}
    };

    let mut walls: Vec<Wall> = Vec::new();
    walls.push(test_wall);

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
        c_line.draw(&mut tex_data, &camera);
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
