
use crate::{Element, vector};
use crate::shapes::Color;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Canvas{
    pub w: u32,
    pub h: u32,
    pub pixels: Vec<Vec<Element>>
}


impl Canvas {
    pub fn new(w0: u32, h0:u32) -> Canvas {
        Canvas{
            w: w0,
            h: h0,
            pixels: vec![vec![vector(0.0,0.0,0.0); w0 as usize];h0 as usize], //[w0] x h0
                //vector(c);totalpixels as usize]
        }
    }

    pub fn h(&self) -> u32 {
        self.h
    }
    pub fn draw(self) {
        println!("P3");
        println!("{} {}", self.w, self.h);
        println!("255");
        for m in &self.pixels{
            for n in m{
                let e = n.grabloc();
                println!("{} {} {}", e[0], e[1], e[2] );
            } 
        }
    }

    pub fn color(&mut self, x:usize, y:usize, c: Color){//Pixel{   // canvas.color(x,y,color)
        let r = (c.r * 255.0).clamp(0.0, 255.0);
        let g = (c.g * 255.0).clamp(0.0, 255.0);
        let b = (c.b * 255.0).clamp(0.0, 255.0);
        self.pixels[y][x] = Element::new(r,g,b,0.0);
        
    }

    
}