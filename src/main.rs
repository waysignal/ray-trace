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
use rays::{Ray,hit};
use shapes::{Sphere, Material,Color, PointLight,normal_at,lighting,Computations,World, Camera, view_transform,render};






fn main() {
    let mut floor = Sphere::new();
    floor.transform = scale(10.0,0.05,10.0);
    floor.material.color = Color::new(1.0,0.9,0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::new();
    left_wall.transform = translation(0.0,0.0,5.0).dot(rotate_y(-PI/4.0).dot(rotate_x(PI/2.0).dot(scale(10.0,0.05,10.0)).unwrap()).unwrap()).unwrap();
    left_wall.material = floor.clone().material;

    let mut right_wall = Sphere::new();
    right_wall.transform = translation(0.0,0.0,5.0).dot(rotate_y(PI/5.0).dot(rotate_x(PI/2.0).dot(scale(5.0,0.05,10.0)).unwrap()).unwrap()).unwrap();
    right_wall.material = left_wall.clone().material;

    let mut middle = Sphere::new();
    middle.transform = translation(-0.5,1.0,0.5);
    middle.material.color = Color::new(0.1,1.0,0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new();
    right.transform = translation(1.5, 0.5, -0.5).dot(scale(0.5,0.5,0.5)).unwrap();
    right.material.color = Color::new(0.5,1.0,0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::new();
    left.transform = translation(-1.5, 0.33, -0.75).dot(scale(0.33,0.33,0.33)).unwrap();
    left.material.color = Color::new(1.0,0.8,0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let world = World{ light_source: PointLight::new(point(-10.0,10.0,-10.0),Color::new(1.0,1.0,1.0)),
                        objects: vec![Box::new(right_wall),Box::new(floor),Box::new(left_wall),Box::new(right),Box::new(left), Box::new(middle)] };
    let mut camera = Camera::new(400,200,PI/3.0);
    camera.transform = view_transform(point(0.0,1.5,-5.0), point(0.0,1.0,0.0), vector(0.0,1.0,0.0));
    let canvas = render(camera,world);
    //canvas.draw() //border along bottom and right side have a black bar, perfectly straight so not the spheres that were deformed -> the minus 1 
}





