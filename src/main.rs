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

// Game
const FUNCTION_LIFETIME: i64 = 10000;
const COLOR_GRAD: [u8; 3*256] = [
        12, 7, 135,
        16, 7, 136,
        19, 6, 137,
        22, 6, 138,
        24, 6, 140,
        27, 6, 141,
        29, 6, 142,
        31, 5, 143,
        33, 5, 144,
        35, 5, 145,
        38, 5, 146,
        40, 5, 146,
        42, 5, 147,
        43, 5, 148,
        45, 4, 149,
        47, 4, 150,
        49, 4, 151,
        51, 4, 151,
        53, 4, 152,
        54, 4, 153,
        56, 4, 154,
        58, 4, 154,
        60, 3, 155,
        61, 3, 156,
        63, 3, 156,
        65, 3, 157,
        66, 3, 158,
        68, 3, 158,
        70, 3, 159,
        71, 2, 160,
        73, 2, 160,
        75, 2, 161,
        76, 2, 161,
        78, 2, 162,
        80, 2, 162,
        81, 1, 163,
        83, 1, 163,
        84, 1, 164,
        86, 1, 164,
        88, 1, 165,
        89, 1, 165,
        91, 0, 165,
        92, 0, 166,
        94, 0, 166,
        95, 0, 166,
        97, 0, 167,
        99, 0, 167,
        100, 0, 167,
        102, 0, 167,
        103, 0, 168,
        105, 0, 168,
        106, 0, 168,
        108, 0, 168,
        110, 0, 168,
        111, 0, 168,
        113, 0, 168,
        114, 0, 169,
        116, 0, 169,
        117, 0, 169,
        119, 1, 168,
        120, 1, 168,
        122, 1, 168,
        123, 2, 168,
        125, 2, 168,
        126, 3, 168,
        128, 3, 168,
        129, 4, 167,
        131, 4, 167,
        132, 5, 167,
        134, 6, 167,
        135, 7, 166,
        136, 7, 166,
        138, 8, 166,
        139, 9, 165,
        141, 11, 165,
        142, 12, 164,
        144, 13, 164,
        145, 14, 163,
        146, 15, 163,
        148, 16, 162,
        149, 17, 161,
        150, 18, 161,
        152, 19, 160,
        153, 20, 160,
        155, 21, 159,
        156, 23, 158,
        157, 24, 157,
        158, 25, 157,
        160, 26, 156,
        161, 27, 155,
        162, 28, 154,
        164, 29, 154,
        165, 30, 153,
        166, 32, 152,
        167, 33, 151,
        169, 34, 150,
        170, 35, 149,
        171, 36, 149,
        172, 37, 148,
        173, 38, 147,
        175, 40, 146,
        176, 41, 145,
        177, 42, 144,
        178, 43, 143,
        179, 44, 142,
        180, 45, 141,
        181, 46, 140,
        183, 47, 139,
        184, 49, 138,
        185, 50, 137,
        186, 51, 137,
        187, 52, 136,
        188, 53, 135,
        189, 54, 134,
        190, 55, 133,
        191, 57, 132,
        192, 58, 131,
        193, 59, 130,
        194, 60, 129,
        195, 61, 128,
        196, 62, 127,
        197, 63, 126,
        198, 64, 125,
        199, 66, 124,
        200, 67, 123,
        201, 68, 122,
        202, 69, 122,
        203, 70, 121,
        204, 71, 120,
        205, 72, 119,
        206, 73, 118,
        207, 75, 117,
        208, 76, 116,
        208, 77, 115,
        209, 78, 114,
        210, 79, 113,
        211, 80, 112,
        212, 81, 112,
        213, 83, 111,
        214, 84, 110,
        215, 85, 109,
        215, 86, 108,
        216, 87, 107,
        217, 88, 106,
        218, 89, 105,
        219, 91, 105,
        220, 92, 104,
        220, 93, 103,
        221, 94, 102,
        222, 95, 101,
        223, 96, 100,
        224, 98, 99,
        224, 99, 98,
        225, 100, 98,
        226, 101, 97,
        227, 102, 96,
        227, 104, 95,
        228, 105, 94,
        229, 106, 93,
        230, 107, 92,
        230, 108, 92,
        231, 110, 91,
        232, 111, 90,
        232, 112, 89,
        233, 113, 88,
        234, 114, 87,
        235, 116, 86,
        235, 117, 86,
        236, 118, 85,
        237, 119, 84,
        237, 121, 83,
        238, 122, 82,
        238, 123, 81,
        239, 124, 80,
        240, 126, 80,
        240, 127, 79,
        241, 128, 78,
        241, 129, 77,
        242, 131, 76,
        242, 132, 75,
        243, 133, 74,
        244, 135, 73,
        244, 136, 73,
        245, 137, 72,
        245, 139, 71,
        246, 140, 70,
        246, 141, 69,
        247, 143, 68,
        247, 144, 67,
        247, 145, 67,
        248, 147, 66,
        248, 148, 65,
        249, 149, 64,
        249, 151, 63,
        249, 152, 62,
        250, 154, 61,
        250, 155, 60,
        251, 156, 60,
        251, 158, 59,
        251, 159, 58,
        251, 161, 57,
        252, 162, 56,
        252, 164, 55,
        252, 165, 54,
        252, 166, 54,
        253, 168, 53,
        253, 169, 52,
        253, 171, 51,
        253, 172, 50,
        253, 174, 49,
        254, 175, 49,
        254, 177, 48,
        254, 178, 47,
        254, 180, 46,
        254, 181, 46,
        254, 183, 45,
        254, 185, 44,
        254, 186, 43,
        254, 188, 43,
        254, 189, 42,
        254, 191, 41,
        254, 192, 41,
        254, 194, 40,
        254, 195, 40,
        254, 197, 39,
        254, 199, 39,
        253, 200, 38,
        253, 202, 38,
        253, 203, 37,
        253, 205, 37,
        253, 207, 37,
        252, 208, 36,
        252, 210, 36,
        252, 212, 36,
        251, 213, 36,
        251, 215, 36,
        251, 217, 36,
        250, 218, 36,
        250, 220, 36,
        249, 222, 36,
        249, 223, 36,
        248, 225, 37,
        248, 227, 37,
        247, 229, 37,
        247, 230, 37,
        246, 232, 38,
        246, 234, 38,
        245, 235, 38,
        244, 237, 39,
        244, 239, 39,
        243, 241, 39,
        242, 242, 38,
        242, 244, 38,
        241, 246, 37,
        241, 247, 36,
        240, 249, 33
];

const C: f32 = 0.1;

// // Utility Functions

fn fexp8(x: f32) -> f32 {
    let mut a = 1.0 + x / 256.0;
    for _ in 0..8 {
        a *= a;
    }
    return a;
}

fn fexp16(x: f32) -> f32 {
    let mut a = 1.0 + x / 256.0;
    for _ in 0..16 {
        a *= a;
    }
    return a;
}

fn fsqrt8(x: f32) -> f32 {
    return fexp8(0.5 * x.ln());
}

fn color_from_value(val: f32) -> [u8; 4] {
    let mut i = 0;
    if val > 0.0 && val < 50.0 {
        i = (256.0*(val/50.0)) as usize;
    }
    let c: [u8; 4] = [0, COLOR_GRAD[i*3 + 2], COLOR_GRAD[i*3 + 1], COLOR_GRAD[i*3 + 0]]; // a, b, g, r
    return c;
}

// // Traits
pub trait Draw {
    fn draw(&self, tex: &mut [u8; (TEX_H*TEX_W*4) as usize], cam: &Camera);
    fn on_screen(&self, cam: &Camera) -> bool;
}

pub trait Move {
    fn move_amount(&mut self, dx: i32, dy: i32);
    fn move_to(&mut self, x: i32, y: i32);
}

pub trait Walled {
    fn get_lines(&self) -> Vec<Line>;
}

pub trait Function {
    fn func(&self, x: i32, y: i32, t: i64) -> f32;
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

#[derive(Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32
}

#[derive(Copy, Clone)]
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

pub enum State {
    Dashing,
    Sliding,
    Jumping,
    Stopping,
    Static
}

struct Player {
    agent: Agent,
    states: Vec<State>
}

pub struct Camera {
    x: i32,
    y: i32,
}

pub struct FunctionStart {
    x0: i32,
    y0: i32,
    t0: i64
}

pub struct PointStart {
    f_start: FunctionStart,
    dx: f32,
    dy: f32
}

pub enum FieldFunction {
    PointFunc(PointStart),
    CircleWave(FunctionStart)
}


// Impl

impl Agent {
    
    fn move_agent_to(&mut self, x: i32, y: i32, walls: &Vec<EnvObject>) -> bool {
        let mut coll = false;
        let movement = Line {x1: self.x + self.mesh.dx, y1: self.y + self.mesh.dy, x2: x + self.mesh.dx, y2: y + self.mesh.dy};
        let mut l = movement.lenght();
        let p_end = Point {x: movement.x2, y: movement.y2};
        let mut lines: Vec<Line> = Vec::new();
        let mn = movement.direction_vector();

        for wall in walls.iter() {
            for line in wall.get_lines().iter() {
                if line.in_reach(movement) {
                    lines.push(*line);
                }
            }
        }

        let mut part_m = Point {x: movement.x1, y: movement.y1};
        'steps: while l > 0.0 {
            if l > self.mesh.r as f32 {
                part_m.x += (mn[0] * (self.mesh.r - 2) as f32) as i32;
                part_m.y += (mn[1] * (self.mesh.r - 2) as f32) as i32;
            } else {
                part_m = p_end;
            }

            'wall_check: for line in lines.iter() {
                let d = line.distance_to_point(&part_m);
                if d < self.mesh.r as f32 {
                    let n = line.normal_vector();
                    let dx = (self.mesh.r as f32 - d)*n[0];
                    let dy = (self.mesh.r as f32 - d)*n[1];
                    
                    if dx > 0.0 { part_m.x += (dx + 1.5) as i32; }
                    else { part_m.x += (dx - 1.5)  as i32; }
                    
                    if dx > 0.0 { part_m.y += (dy + 1.5) as i32; }
                    else { part_m.y += (dy - 1.5)  as i32; }

                    coll = true;
                    break 'steps;
                } else {
                    continue 'wall_check;
                }
            }
            l -= self.mesh.r as f32 * 0.5;
        }

        self.x = part_m.x - self.mesh.dx;
        self.y = part_m.y - self.mesh.dy;
        self.tex.move_to(self.x, self.y);

        return coll;
    }

    fn move_agent_amount(&mut self, dx: i32, dy: i32, walls: &Vec<EnvObject>) -> bool {
        return self.move_agent_to(self.x + dx, self.y + dy, walls);
    }

}

impl Player {
    fn move_player(&mut self, dt: i64, walls: &Vec<EnvObject>) {

        self.agent.move_agent_amount((self.agent.vx*dt as f32) as i32, (self.agent.vy*dt as f32) as i32, walls);
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

    fn swap_points(&self) -> Line {
        return Line {x1: self.x2, y1: self.y2, x2: self.x1, y2: self.y1};
    }

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
        let v = self.direction_vector();
        return [v[1], -v[0]];
    }

    fn direction_vector(&self) -> [f32; 2] {
        let n = self.lenght();
        let x = (self.x2-self.x1) as f32 / n;
        let y = (self.y2-self.y1) as f32 / n;
        return [x, y];
    }

    fn in_reach(&self, l: Line) -> bool {
        let ld = l.direction_vector();
        let n = self.normal_vector();
        if ld[0]*n[0] + ld[1]*n[1] > 0.0 {
            return false;
        }
        /*
        let v0 = [(l.x2 - l.x1) as f32, (l.y2 - l.y1) as f32];
        let v1 = [(self.x1 - l.x1) as f32, (self.y1 - l.y1) as f32];
        let v2 = [(self.x2 - l.x1) as f32, (self.y2 - l.y1) as f32];
        if v0[0]*v1[0] + v0[1]*v1[1] < 0.0 && v0[0]*v2[0] + v0[1]*v2[1] < 0.0 {
            return false;
        }
        let d = self.distance_to_point(&Point {x: l.x1, y: l.y1});
        if d > l.lenght(){
            return false;
        }*/
        return true;
    }

}

impl Wall {
    fn get_lines(&self) -> Vec<Line> {
        let mut v: Vec<Line> = Vec::new();
        v.push(self.line);
        v.push(self.line.swap_points());
        return v;
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
        let mut line = self.line;

        let lx = line.x2-line.x1;
        let ly = line.y2-line.y1;
        if lx.abs() > ly.abs() {
            if line.x2 < line.x1 {
                line = line.swap_points();
            }
            let dy = ly as f32 / lx as f32;
            for x in 0..lx {
                let sy = (line.y1 as f32 + dy * x as f32) as i32 - cam.y;
                let sx = line.x1 + x - cam.x;

                if sx < 0 || sx + 1 > TEX_W || sy < 0 || sy + 1 > TEX_H {
                    continue;
                }

                for c in 0..4 {
                    tex[(((sy as i32)*TEX_W + sx)*4 + c) as usize] = self.col[c as usize];
                }
            }
        } else {
            if line.y2 < line.y1 {
                line = line.swap_points();
            }
            let dx = lx as f32 / ly as f32;
            for y in 0..ly {
                let sx = (line.x1 as f32 + dx * y as f32) as i32 - cam.x;
                let sy = line.y1 + y - cam.y;

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

impl Walled for EnvObject {
    fn get_lines(&self) -> Vec<Line> {
        match self {
            EnvObject::Wall(wall) => {
                return wall.get_lines();
            },
            _ => {
                let v: Vec<Line> = Vec::new();
                return v;
            }
        }
    }
}

impl Function for FieldFunction {
    fn func(&self, x: i32, y: i32, t: i64) -> f32 {
        match self {
            FieldFunction::PointFunc(start) => {
                let t1 = t-start.f_start.t0;
                if t1 > FUNCTION_LIFETIME {
                    return 0.0;
                }

                let x1 = x as f32 - start.f_start.x0 as f32 - C*t1 as f32*start.dx;
                let y1 = y as f32 - start.f_start.y0 as f32 - C*t1 as f32*start.dy;
                let r = (x1*x1 + y1*y1).sqrt();
                let s = 20.0;
                if r.abs() > s {
                    return 0.0;
                }
                return s * fexp8(-r*r*(1.0/s));
            },
            FieldFunction::CircleWave(start) => {
                let x1 = x-start.x0;
                let y1 = y-start.y0;
                let t1 = t-start.t0;
                if t1 > FUNCTION_LIFETIME {
                    return 0.0;
                }
                let r = ((x1*x1 + y1*y1) as f32).sqrt() - C*t1 as f32;
                let s = 20.0;
                if r.abs() > s {
                    return 0.0;
                }
                return s * fexp8(- r*r*(1.0/s));
            },
            _ => {
                return 0.0;
            }
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
    let mut dt: i64 = 0;
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
        col: [255, 0, 0, 255]
    };

    let player_sqr = ColorSquare {
        sqr: Square {
            x: 0,
            y: 0,
            w: 10,
            h: 10
        },
        col: [255, 250, 100, 55]
    };

    let c_line1 = ColorLine {
        line: Line {
            x1: 200,
            y1: 200,
            x2: 500,
            y2: 700
        },
        col: [255, 50, 250, 100]
    };

    let test_wall1 = Wall {
        line: Line {
            x1: 200,
            y1: 200,
            x2: 500,
            y2: 700
        }
    };


    let mut camera = Camera {x: 0, y: 0};
    
    let mut player_agent = Agent{
        x: 0,
        y: 0,
        vx: 0.0,
        vy: 0.0,
        speed: 0.3,
        tex: Texture::Square(player_sqr),
        mesh: Mesh {dx: 5, dy: 5, r: 5}
    };

    let mut player = Player {
        agent: player_agent,
        states: Vec::new()
    };

    let mut walls: Vec<EnvObject> = Vec::new();
    walls.push(EnvObject::Wall(test_wall1));

    let mut f_list: Vec<FieldFunction> = Vec::new();
    let tmp1 = FieldFunction::CircleWave(FunctionStart {x0: 0, y0: 0, t0: 0});
    let tmp2 = FieldFunction::PointFunc(PointStart{f_start: FunctionStart {x0: 0, y0: 0, t0: 0}, dx: 0.4 , dy: 0.4 });
    f_list.push(tmp1);
    f_list.push(tmp2);

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
                    player.agent.vx = -player.agent.speed;
                },
                Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                    if player.agent.vx < 0.0 {
                        player.agent.vx = 0.0;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                     player.agent.vx = player.agent.speed;
                },
                Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                    if player.agent.vx > 0.0 {
                        player.agent.vx = 0.0;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    player.agent.vy = -player.agent.speed;
                },
                Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                    if player.agent.vy < 0.0 {
                        player.agent.vy = 0.0;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    player.agent.vy = player.agent.speed;
                },
                Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                    if player.agent.vy > 0.0 {
                        player.agent.vy = 0.0;
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


        //println!("{}, {}", (player.vx*dt as f32) as i32, (player.vy*dt as f32) as i32);

        camera.x = player.agent.x - TEX_W/2 + player.agent.mesh.dx;
        camera.y = player.agent.y - TEX_H/2 + player.agent.mesh.dy;
        
        let mut max_val = 0.0;
        for i in 0..TEX_W {
            for j in 0..TEX_H {
                let mut val = 0.0;
                let x = camera.x + i;
                let y = camera.y + j;
                for f in f_list.iter() {
                    val += f.func(x, y, t);
                }
                if val > max_val {
                    max_val = val;
                }
                let a = color_from_value(val);
                for k in 0..4 {
                    tex_data[((TEX_W*j + i)*4 + k) as usize] = a[k as usize];
                    // tex_data[((TEX_W*j + i)*4 + k) as usize] = 0;
                }
            }
        }

        player.move_player(dt, &walls);

        test_sqr.draw(&mut tex_data, &camera);
        c_line1.draw(&mut tex_data, &camera);
        player.agent.tex.draw(&mut tex_data, &camera);

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
