
use crate::{Element, vector};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Canvas{
    pub w: u64,
    pub h: u64,
    pub pixels: Vec<Vec<Element>>
}


impl Canvas {
    pub fn new(w0: u64, h0:u64) -> Canvas {
        Canvas{
            w: w0,
            h: h0,
            pixels: vec![vec![vector(0.0,0.0,0.0); w0 as usize];h0 as usize], //[w0] x h0
                //vector(c);totalpixels as usize]
        }
    }

    pub fn h(&self) -> u64 {
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

    pub fn color(&mut self, x:usize, y:usize, c: Element){//Pixel{   // canvas.color(x,y,color)
        self.pixels[y][x] = c;
        
    }

}