use crate::{Element,Matrix,point,vector};
use std::{ops::{Index, Add, Sub, Neg, Mul, Div}, vec};
#[derive(Debug, Clone,PartialEq)]
pub struct Sphere{
    pub loc: Element,
    pub transform: Matrix,
    pub material: Material,
}

impl Sphere{
    pub fn new() -> Sphere{
        Sphere{loc: point(0.0,0.0,0.0),
            transform: Matrix::zero(4,4).identity(),material: Material::new()}
    }
    pub fn set_transform(&mut self,t: &Matrix){
        let t_c = t.clone();
        self.transform = t_c;

    }

    pub fn get_material(self) -> Material {
        self.material
    }
}

pub fn normal_at(obj: &Sphere, pos: &Element) -> Element{
    let pos_0 = pos.clone(); //using clone to dereference pos (cant just do it bc it is shared), dont need to for obj bc by accessing its field, it already does so -> edit: jk need for obj too
    let obj_0 = obj.clone();
    let object_poi = obj_0.transform.invert().unwrap().dot(pos_0.matrix).unwrap(); //poi in object space
    let object_nor = (Element{matrix:object_poi} - point(0.0,0.0,0.0)); //get normal dir of (poi and sphere) in object space, hint: finding normal from 0,0,0 (sphere)
    let mut world_nor = obj_0.transform.invert().unwrap().transpose().dot(object_nor.matrix).unwrap(); //
    world_nor.matrix[3][0] = 0.0;
    Element{matrix: world_nor}.normal()

}

pub fn reflect(i: &Element, nor: &Element) -> Element{
    let i = i.clone();
    let nor = nor.clone();
    //difference in dot functions for matrix and element
    i.clone() - nor.clone() * (2.0 * i.dot(nor))
}

#[derive(Debug, Clone,PartialEq)]
pub struct PointLight{
    pub intensity: Color,
    pub position: Element,
}
impl PointLight{
    pub fn new(pos: Element, int: Color) -> PointLight{
        PointLight{
            intensity: int,
            position: pos,
        }
    }
}

#[derive(Debug, Clone,PartialEq)]
pub struct Material{
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material{
    pub fn new() -> Material{
        Material { color: Color::new(1.0,1.0,1.0), ambient: 0.1 , diffuse: 0.9, specular: 0.9, shininess: 200.0 }
    }
}



pub fn lighting(m: Material,light:&PointLight,position: Element,eyev: Element,normalv:Element) -> Color {
    //let e = Element {matrix: m.color.matrix.dot(light.clone().intensity.matrix).unwrap()};
    
    let light = light.clone();
    let effective_color = m.color * light.clone().intensity;
    let lightv = (light.clone().position - position).normal();
    let ambient =  effective_color.clone() * m.ambient ;
    let light_dot_normal = lightv.dot(normalv.clone());
    let mut diffuse = Color::new(0.0,0.0,0.0);
    let mut specular = Color::new(0.0,0.0,0.0);
    

    if light_dot_normal < 0.0 {
        diffuse =  Color::new(0.0,0.0,0.0);
        specular =  Color::new(0.0,0.0,0.0);
    } else {
        diffuse = effective_color * m.diffuse * light_dot_normal;
        let reflectv = reflect(&-lightv, &normalv);
        let reflect_dot_eye = reflectv.dot(eyev);
        if reflect_dot_eye <= 0.0 {
            specular =  Color::new(0.0,0.0,0.0);
        } else {
            let factor = reflect_dot_eye.powf(m.shininess);
            specular = light.intensity * m.specular * factor;
        }

    }

    ambient + diffuse + specular 
}
#[derive(Debug, Clone,PartialEq)]
pub struct Color{
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color{
    pub fn new(r: f32, g: f32, b: f32) -> Color{
        Color { r: r, g: g, b: b }
    }
}

impl Sub for Color{
    type Output = Color;
    fn sub(self, other: Color) -> Color{
        Color { r: self.r - other.r,
                g: self.g - other. g, 
                b: self.b - other.b }
    }
}


impl Add for Color{
    type Output = Color;
    fn add(self, other: Color) -> Color{
        Color { r: self.r + other.r,
                g: self.g + other. g, 
                b: self.b + other.b }
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, other: f32) -> Color{
        Color { r: self.r * other,
            g: self.g * other, 
            b: self.b * other }
    }
}


impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color{
        Color { r: self.r * other.r,
            g: self.g * other.g, 
            b: self.b * other.b }
    }
}