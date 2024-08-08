use core::ops::{Add, Sub,Mul, Div};
use std::f64::consts::PI;
use raylib::{prelude::color::Color, prelude};
#[derive(Clone, Copy,PartialEq, Debug)]
pub struct Vector2 {
    pub x:f64,
    pub y:f64
}

impl Add for Vector2{
    type Output = Self;
    fn add(self, other:Self)->Self{
        return Self{x:self.x+other.x, y:self.y+other.y};
    }
    
}

impl Sub for Vector2{
    type Output = Self;
    fn sub(self, other:Self)->Self{
        return Self{x:self.x-other.x, y:self.y-other.y};
    }
}

impl Mul<f64> for Vector2{
    type Output = Self;
    fn mul(self,other:f64)->Self{
        return Self{x:self.x*other, y:self.y*other};
    }
}

impl Div<f64> for Vector2{
    type Output = Self;
    fn div(self, other:f64)->Self{
        return Self{x:self.x/other, y:self.y/other};  
    }
}

impl Vector2{
    pub fn new(x:f64, y:f64)->Self{
        return Vector2 { x: x, y: y };
    }
    pub fn zero()->Self{
        return Vector2{x:0.0, y:0.0};
    }
}

pub struct AxisAlignedRect{
    pub x:f64, 
    pub y:f64, 
    pub w:f64, 
    pub h:f64,
}

impl AxisAlignedRect{
    pub fn contains(&self,point:Vector2)->bool{
        return false;
    }
}

pub fn base_distrib_func(point:Vector2)->f64{
    fn func(base:f64)->f64{
        let scale:f64 = 1.0;
        let sx =base*scale;
        return f64::exp(-PI*sx*sx)*scale;
    }
    func(point.x)*func(point.y)
}

pub struct StateVec{
    pub location:Vector2,
    pub velocity:Vector2,
    density_func:fn (Vector2)->f64
}
impl StateVec{
    pub fn new(location:Vector2, velocity:Vector2)->Self{
        return Self{location, velocity, density_func:base_distrib_func};
    }
    pub fn sample_density(&self,point:Vector2)->f64{
        let p = point-self.location;
        return ((self.density_func))(p);
    }
    pub fn get_density_func(&self)-> impl Fn (Vector2)->f64{
        let density_func = self.density_func;
        let location = self.location;
        let out = move |point:Vector2|->f64{(density_func)(point-location)};
        return out
    }
}

pub fn area_integration(rect:AxisAlignedRect,resolution:f64,function:&impl Fn(Vector2)->f64)->f64{
    fn single_area_calc(x0:f64,y0:f64,x1:f64,y1:f64,disp_x: f64,disp_y: f64, function:&impl Fn(Vector2)->f64)->f64{
        let mid_x = (x0+x1)/2.0;
        let mid_y = (y0+y1)/2.0;
        return function(Vector2::new(mid_x, mid_y))*disp_x*disp_y;
    }
    let division_count_x = (rect.w*resolution).ceil() as usize;
    let division_count_y = (rect.h*resolution).ceil() as usize;
    let mut out = 0.0;
    let disp_x = rect.w/(division_count_x as f64);
    let disp_y = rect.h /(division_count_y as f64);
    for i in 0..division_count_x{
        for j in 0..division_count_y{
            let x0 = rect.x+i as f64*disp_x;
            let y0 = rect.y+j as f64*disp_y;
            let x1 = x0+disp_x;
            let y1 = y0+disp_y;
            let v = single_area_calc(x0, y0, x1, y1,disp_x, disp_y, function);
            out += v;
        }
    }
    return out;
}

pub fn test(height:i32, width:i32){
    let mut out = prelude::Image::gen_image_color(width,height, Color::BLACK);
    let state = StateVec::new(Vector2::zero(), Vector2::zero());
    let tmp = state.get_density_func();
    let scale = 0.005;
    for y0 in 0..height{
        for x0 in 0..width{
            let x = x0 as f64-width as f64/2.0;
            let y = y0 as f64-height as f64/2.0;
            let v = Vector2::new(x,y)*scale;
            let p = tmp(v);
            let col = (p *255.0) as u8;
            out.draw_pixel(x0,y0, Color{r:col,g:col, b:col, a:255});
        }
    }
    let area = area_integration(AxisAlignedRect{x:-500.0*scale, y:-500.0*scale, w:1000.0*scale, h:1000.0*scale},2.0/scale,&tmp);
    println!("{area}");
    out.export_image("test.png");
}

fn main() {/* 
    let mut tmp = raylib::prelude::init().size(1000, 1000).build();
    let mut thread = tmp.1;
    let mut handle = tmp.0;
    handle.begin_drawing(&thread);*/
    test(1000,1000);
}
