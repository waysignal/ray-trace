mod tuples;
mod canvas;
mod matrix;
mod transformation;
mod test;
mod rays;
mod shapes;

use std::{f64::consts::PI, ops::Add};
use tuples::{Element, vector, point};
use canvas::{Canvas};
use matrix::matrix::*;
use rays::{Ray,hit};
use shapes::{*};






fn main() {
    let mut floor = Sphere::new();
    floor.transform = scale(10.0,0.01,10.0);
    floor.material.color = Color::new(0.5,0.5,0.9);
    floor.material.specular = 0.0;
    floor.material.transparency = 0.9;
    floor.material.reflective = 0.9;

    let mut left_wall = Sphere::new();
    left_wall.transform = translation(0.0,0.0,5.0).dot(rotate_y(-PI/4.0).dot(rotate_x(PI/2.0).dot(scale(10.0,0.01,10.0)).unwrap()).unwrap()).unwrap();
    left_wall.material = floor.clone().material;

    let mut right_wall = Sphere::new();
    right_wall.transform = translation(0.0,0.0,5.0).dot(rotate_y(PI/4.0).dot(rotate_x(PI/2.0).dot(scale(10.0,0.01,10.0)).unwrap()).unwrap()).unwrap();
    right_wall.material = left_wall.clone().material;

    let mut middle = Sphere::new();
    middle.transform = translation(-0.5,1.0,0.5);
    middle.material.color = Color::new(0.1,1.0,0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    middle.material.pattern = Some(CheckersPattern::new(Color::new(0.1,1.0,0.5), Color::white()).box_me());
    middle.material.reflective = 0.5;

    let mut right = Sphere::new();
    right.transform = translation(1.5, 0.5, -0.5).dot(scale(0.5,0.5,0.5)).unwrap();
    right.material.color = Color::new(0.5,1.0,0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::new();
    left.transform = translation(-1.5, 0.33, -0.75).dot(scale(0.33,1.33,0.33)).unwrap();
    left.material.color = Color::new(1.0,0.8,0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.9;
    left.material.shininess = 300.0;
    left.material.transparency = 0.3;
    left.material.reflective = 0.9;
    
    let mut c = Cube::new();
    c.transform = translation(3.0, 0.0, 4.0).dot(scale(0.5,0.5,1.0)).unwrap();
    c.material.color = Color::new(0.95,0.1,0.1);
    c.material.reflective = 0.9;

    let mut cone = Cone::new();
    cone.transform = translation(-2.0, 1.0, 4.0).dot(scale(1.33,1.33,1.0)).unwrap();
    cone.min = -1.0;
    cone.max = 0.0;
    cone.material.color = Color::new(0.5,0.8,0.1);
    cone.material.transparency = 0.3;
    cone.closed = true;
    
    let mut p = Plane::new();
    p.material.specular = 0.5;
    p.material.ambient = 0.0; //
    p.material.transparency = 1.0;
    p.material.reflective = 0.0;
    
    p.material.pattern = Some(CheckersPattern::new(Color::white(), Color::new(0.4,0.5,0.9)).box_me());

    let world = World{ light_source: PointLight::new(point(-10.0,10.0,-10.0),Color::new(1.0,1.0,1.0)),
                        objects: vec![Box::new(cone),Box::new(p),Box::new(right), Box::new(middle), Box::new(c)] };
    let mut camera = Camera::new(800,400,PI/3.0);
    camera.transform = view_transform(point(-2.0,3.5,-5.0), point(0.0,1.0,0.0), vector(0.0,1.0,0.0));
    let canvas = render(camera,world);
    canvas.draw() //lighting is off for the walls, not spheres. transformation error? 
}





