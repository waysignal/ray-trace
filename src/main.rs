mod tuples;
mod canvas;
mod matrix;
mod transformation;
mod test;
mod rays;
mod shapes;

use std::{f32::consts::PI, ops::Add};
use tuples::{Element, vector, point};
use canvas::{Canvas};
use matrix::matrix::*;
use rays::{Ray,intersect,hit};
use shapes::{Sphere, Material,Color, PointLight,normal_at,lighting};






fn main() {
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 400.0;
    let pixel_size = wall_size/canvas_pixels;
    let half = wall_size/2.0;

    let mut new = Canvas::new(400,400);
    let mut s = Sphere::new();
    s.material.color = Color::new(1.0,0.2,1.0);
   
    let light = PointLight::new(point(-10.0,10.0,-10.0), Color::new(1.0,1.0,1.0));

    // let t = shearing(1.0,0.0,0.0,0.0,0.0,0.0).dot(scale(0.5,1.0,1.0)).unwrap();
    // s.set_transform(&t);

    for y in 0..400 {
        let world_y = half - pixel_size * (y as f32);

        for x in 0..400 {
            let world_x = -half + pixel_size * (x as f32);
            let position = point(world_x, world_y, wall_z);

            let r = Ray::new(point(0.0,0.0,-5.0), (position - point(0.0,0.0,-5.0).normal()).normal());
            let mut xs = intersect(&r, &s);
            if xs.count > 0 {
                let int = hit(&mut xs).unwrap();
                let int_s = int.clone().o.clone();      /// BYPASSING THE SHARED REFERENCES
                let point = r.position(int.t);
                let normal = normal_at(&int.o, &point);
                let eye = -r.dir();
                let color = lighting(int_s.material, &light, point, eye, normal);
                new.color(x,y,color);
            }
        }

    }

    new.draw();
}





