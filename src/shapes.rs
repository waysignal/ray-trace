use crate::canvas::Canvas;
use crate::{Element,Matrix,point,vector,scale, equal_floats,translation};
use crate::rays::{Ray, Intersections,Intersection,hit};
use std::{ops::{Index, Add, Sub, Neg, Mul, Div}, vec};

static EPSILON: f32 = 0.005;


pub trait ShapeThings{
    fn intersect(&self,r: &Ray) -> Intersections;
    fn get_transform(&self) -> Matrix;
    fn normal_at(&self, pos: &Element) -> Element;
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub transform: Matrix,
    pub material: Material,
    
}

impl Shape {
    pub fn set_transform(&mut self,t: Matrix){ //switching to not a reference for Matrix bc design choice: each one should be unique
        let t_c = t.clone();
        self.transform = t_c;

    }

    pub fn test() -> Shape{
        Shape{
            transform: Matrix::zero(4,4).identity(),
            material: Material::new(),
            
        }
    }
}

impl ShapeThings for Shape{
    
    fn intersect(&self,r: &Ray) -> Intersections{

        eprintln!("{:?}", r.clone().transform(&self.transform.invert().unwrap()));
        Intersections { count: 0, h: vec![] }
    }
    fn get_transform(&self) -> Matrix {
        self.clone().transform
    }
    fn normal_at(&self, pos: &Element) -> Element{
        let mut p = pos.clone();
        p.matrix.matrix[3][0] = 0.0;
        p
    }
}

#[derive(Debug,Clone)]
struct Plane{
    pub transform: Matrix,
    pub material: Material,
}

impl ShapeThings for Plane{
    fn intersect(&self,r: &Ray) -> Intersections{
        if (r.direction.y()).abs() < EPSILON{
            Intersections { count: 0, h: vec![]}
        } else {
            let t = -r.origin.y()/r.direction.y();
            Intersections { count: 1, h: vec![Intersection{ t: t, o: self}] }
        }
    }
    fn get_transform(&self) -> Matrix {
        self.clone().transform
    }
    fn normal_at(&self, pos: &Element) -> Element{
        vector(0.0,1.0,0.0)
    }
}
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

impl ShapeThings for Sphere{
    fn intersect(&self,r: &Ray) -> Intersections{
        let t_r = r.clone().transform(&self.transform.invert().unwrap()); //why ref here too?, isnt obj already one?, accessing fields changes this?
        
        //maybe have dot use reference so we dont have to keep repeating clone
        let a = t_r.clone().direction.dot(t_r.clone().direction);
        let b = 2.0 * t_r.clone().direction.dot(t_r.clone().sphere_to_ray(&self));
        let c = t_r.clone().sphere_to_ray(&self).dot(t_r.clone().sphere_to_ray(&self)) - 1.0; 
        //radius is still 1 bc we are scaling the ray, operating in object space
        // eprintln!("t_r: {:?}", t_r); //correct
        // eprintln!("sphere loc: {:?}", obj.loc);
        // eprintln!("sphere to ray: {:?}", t_r.clone().sphere_to_ray(&obj));
        // eprintln!("a: {:?}, \n b: {:?} \n c: {:?} ", a,b,c);
            //a is modulus squared
            //b 
        let discri = b.powi(2) - 4.0 * a * c;
        if discri < 0.0 {
            Intersections { count: 0,  h: vec![]}
        } else {
            let t1 = (-b - discri.sqrt())/(2.0*a);
            let t2 = (-b + discri.sqrt())/(2.0*a);
            //even if the t is in object space, why should it be different? the direction has been scaled as well so it should cancel out
            let s1 =  t_r.position(t1).z();
            let s2 =  t_r.position(t2).z(); //s's are positions of intersections
            //distance away is given by position()
            let i1 = Intersection::new(t1,&self);
            let i2 = Intersection::new(t2,&self);
            Intersections { count: 2,  h: vec![i1,i2]}            
            }
        }
    fn get_transform(&self) -> Matrix {
        self.clone().transform
    }

    fn normal_at(&self, pos: &Element) -> Element {
        let pos_0 = pos.clone(); //using clone to dereference pos (cant just do it bc it is shared), dont need to for obj bc by accessing its field, it already does so -> edit: jk need for obj too
        let obj_0 = self.clone();
        let object_poi = obj_0.transform.invert().unwrap().dot(pos_0.matrix).unwrap(); //poi in object space
        let object_nor = Element{matrix:object_poi} - point(0.0,0.0,0.0); //get normal dir of (poi and sphere) in object space, hint: finding normal from 0,0,0 (sphere)
        let mut world_nor = obj_0.transform.invert().unwrap().transpose().dot(object_nor.matrix).unwrap(); //
        world_nor.matrix[3][0] = 0.0;
        Element{matrix: world_nor}.normal()
    }
    
}

pub fn normal_at<S>(obj: &S, pos: &Element) -> Element
    where S: ShapeThings{
    let local_point = Element{ matrix: obj.get_transform().invert().unwrap().dot(pos.clone().matrix).unwrap()}; //poi in object space
    let local_normal = obj.normal_at(&local_point);
    let mut world_nor = obj.get_transform().invert().unwrap().transpose().dot(local_normal.matrix).unwrap(); //
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



pub fn lighting(m: Material,light:&PointLight,position: Element,eyev: Element,normalv:Element,in_shadow:bool) -> Color {
    //let e = Element {matrix: m.color.matrix.dot(light.clone().intensity.matrix).unwrap()};
    
    let light = light.clone();
    let effective_color = m.color * light.clone().intensity;
    let lightv = (light.clone().position - position).normal();
    let ambient =  effective_color.clone() * m.ambient ;
    let light_dot_normal = lightv.dot(normalv.clone());
    let mut diffuse = Color::new(0.0,0.0,0.0);
    let mut specular = Color::new(0.0,0.0,0.0);
    

    if light_dot_normal < 0.0  {
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
    if in_shadow {
        ambient
    }else {
        ambient + diffuse + specular 
    }
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
    pub fn equal(self, o: Color) -> bool {
        
        let l = vec![equal_floats(&self.r,&o.r), equal_floats(&self.g,&o.g) , equal_floats(&self.b,&o.b)];
        if l.contains(&false){
            false
        }else{
            true
        }
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

#[derive(Debug,Clone)]
pub struct World{
    pub objects: Vec<Sphere>,
    pub light_source: PointLight,
}

impl World{
    pub fn new() -> World {
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8,1.0,0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::new();
        s2.set_transform(&scale(0.5,0.5,0.5));
        World { objects: vec![s1,s2] , light_source: PointLight::new(point(-10.0,10.0,-10.0), Color::new(1.0,1.0,1.0)) }
    }

    pub fn intersect_world<'a>(&'a self,r:&Ray,mut l:  Vec<Intersection<'a>>) -> Intersections<'a>{

        for (i,j) in self.objects.iter().enumerate(){
            for (a,b) in j.intersect(r).h.iter().enumerate(){   //CHECK 
                let hits = b.clone();
                l.push(hits);
            }
        }
        l.sort_by(|e1 ,e2| e1.t.partial_cmp(&e2.t).unwrap());
        let list_l= l.len() as u32;
        let list_s = l.clone();
        Intersections{ 
            count: list_l,
            h: list_s,
        }
    }
}

#[derive(Debug,Clone)]
pub struct Computations{
    pub t: f32,
    pub object: Sphere,
    pub point: Element,
    pub eyev: Element,
    pub normalv: Element,
    pub inside: bool,
    pub over_point: Element

}

impl Computations{
    pub fn prepare_computations(i: &Intersection, r: &Ray) -> Computations
    {
        let t = i.t;
        let object = i.o;
        let point =r.position(t);
        let mut normalv = normal_at(object, &point);
        let eyev= -r.clone().direction;
        let mut inside = true;
        if normalv.clone().dot(eyev.clone()) < 0.0 {

            normalv = -normalv;

        }
        else {
            inside = false;
        }
        
        let c =Computations{
            t: i.t,
            object: i.o.clone(),
            point: point.clone(),
            eyev: eyev,
            normalv: normalv.clone(), //how to use clone less?
            inside: inside,
            over_point : point.clone() + normalv.clone() * EPSILON,
        };

        c
        
    }
}


pub fn shade_hit(world: &World, comps: Computations) -> Color{
    let shadowed = is_shadowed(world, &comps.over_point);
    lighting(comps.object.material, &world.light_source, comps.point, comps.eyev, comps.normalv,shadowed)
}

pub fn color_at(w: &World, r: &Ray) -> Color{
    let mut intersections = w.intersect_world(r, vec![]);
    let hit = hit(&mut intersections);

    if hit == None {
        Color::new(0.0,0.0,0.0)
    } else {
        let comp = Computations::prepare_computations(hit.unwrap(),r);
        shade_hit(w,comp)


    }

}

pub fn view_transform(from: Element ,to: Element , up: Element) -> Matrix {
    let forward = (to-from.clone()).normal(); 
    let upn = up.normal();
    let left = forward.clone().cross(upn);
    let true_up = left.cross(forward.clone());
    let mut orientation = Matrix::new(vec![vec![left.x(),left.y(),left.z(),0.0],
                                                vec![true_up.x(),true_up.y(),true_up.z(),0.0],
                                                vec![-forward.x(),-forward.y(),-forward.z(),0.0],
                                                vec![0.0,0.0,0.0,1.0]]);
    orientation.dot(translation(-from.x(),-from.y(),-from.z())).unwrap()

}

#[derive(Debug)]
pub struct Camera{
    pub hsize: u32,
    pub vsize: u32,
    pub field_of_view: f32,
    pub transform: Matrix,
    pub pixel_size: f32,
    pub half_width: f32,
    pub half_height: f32,

}

impl Camera{
    pub fn new(hsize:u32,vsize: u32,
        field_of_view: f32,) -> Camera {
            let half_view = (field_of_view/2.0).tan();
            let aspect = hsize as f32 /vsize as f32;
            let mut half_width;
            let mut half_height;
            if aspect >= 1.0 {
                half_width = half_view;
                half_height=half_view/aspect;
            } else{
                half_width = half_view*aspect;
                half_height= half_view;
            }
            let pixel_size = half_width * 2.0 / hsize as f32;
            Camera { hsize: hsize, vsize: vsize, field_of_view: field_of_view, transform: Matrix::zero(4,4).identity(), pixel_size: pixel_size, half_height: half_height, half_width: half_width}
        }
    
    pub fn ray_for_pixel(&self,px:u32,py:u32) -> Ray {
        let xoffset = (px as f32 +  0.5) * self.pixel_size;
        let yoffset = (py as f32 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset; //f32 has copy trait
        let world_y = self.half_height - yoffset;

        let inverse_t = self.transform.invert().unwrap();
        let pixel = inverse_t.dot(point(world_x,world_y, -1.0).matrix).unwrap();

        let origin = inverse_t.dot(point(0.0,0.0,0.0).matrix).unwrap();
        let direction = (Element{matrix: pixel.clone()} - Element{ matrix: origin.clone()}).normal();
        Ray::new(Element{matrix: origin} , direction)

    }
    
}

pub fn render(c: Camera, w: World) -> Canvas{
    let mut image = Canvas::new(c.hsize,  c.vsize);
    for y in 0..c.vsize {
        for x in 0..c.hsize {
            let ray = c.ray_for_pixel(x, y);
            let color = color_at(&w, &ray);
            image.color(x.try_into().unwrap() ,y.try_into().unwrap(),color);
        }
    }
    image
}

pub fn is_shadowed(world: &World, point: &Element) -> bool {
    let point = point.clone();
    let world = world.clone();
    let v =  world.clone().light_source.position - point.clone();
    let distance = v.magnitude();
    let direction = v.normal();

    let ray = Ray::new(point,direction);
    let mut intersections = world.intersect_world(&ray, vec![]); //removed clone from world -> temp value is now not freed?
    let hit = hit(&mut intersections);
    if hit.is_none() {
        false
    } else {
        let hit = hit.unwrap();
        if hit.t < distance{
            true
        } else {
            false
        }
        
    }

}