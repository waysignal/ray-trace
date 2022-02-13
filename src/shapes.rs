use crate::canvas::Canvas;
use crate::{Element,Matrix,point,vector,scale, equal_floats,translation};
use crate::rays::{Ray, Intersections,Intersection,hit, intersect_shape};
use std::{ops::{Index, Add, Sub, Neg, Mul, Div}, vec};
use std::any::Any;

static EPSILON: f64 = 0.00001;


#[derive(Debug,Clone,PartialEq)]
pub struct CheckersPattern{
    a: Color,
    b: Color,
    transform: Matrix,
}
impl Pattern for CheckersPattern{

        fn box_me(self) -> Box<dyn Pattern>{
            Box::new(self) as Box<dyn Pattern>
        }
    
        fn get_transform(&self) -> Matrix{
            self.clone().transform
        } 
    
        fn set_transform(&mut self,t: Matrix){
            let t_c = t.clone();
            self.transform = t_c;
        }
        fn pattern_at(&self,p: Element) -> Color {
            if (p.x().floor() + p.y().floor() + p.z().floor())%2.0 == 0.0 {
                self.get_a()
            } else{
                self.get_b()
            }
        }
    
        fn pattern_at_shape(&self, obj: & Box<dyn ShapeThings>, world_point: Element) -> Color  {
            let obj_point = obj.get_transform().invert().unwrap().dot(world_point.matrix).unwrap();
            let pattern_point = Element{ matrix: self.get_transform().invert().unwrap().dot(obj_point).unwrap()};
            self.pattern_at(pattern_point)
    
        }
}
impl CheckersPattern{
    pub fn new(a: Color, b: Color) -> CheckersPattern{
        CheckersPattern{a: a, b: b, transform: Matrix::zero(4,4).identity()}
    }
    pub fn base() -> CheckersPattern{
        CheckersPattern{a: Color::white(), b: Color::black(), transform: Matrix::zero(4,4).identity()}
    }
    pub fn get_a(&self) -> Color{
        self.clone().a
    } 
    pub fn get_b(&self) -> Color{
        self.clone().b
    } 
}






#[derive(Debug,Clone,PartialEq)]
pub struct RingPattern{
    a: Color,
    b: Color,
    transform: Matrix,
}
impl Pattern for RingPattern{

        fn box_me(self) -> Box<dyn Pattern>{
            Box::new(self) as Box<dyn Pattern>
        }
    
        fn get_transform(&self) -> Matrix{
            self.clone().transform
        } 
    
        fn set_transform(&mut self,t: Matrix){
            let t_c = t.clone();
            self.transform = t_c;
        }
        fn pattern_at(&self,p: Element) -> Color {
            if (p.x().powf(2.0)+p.z().powf(2.0)).sqrt().floor()%2.0 == 0.0 {
                self.get_a()
            } else{
                self.get_b()
            }
        }
    
        fn pattern_at_shape(&self, obj: & Box<dyn ShapeThings>, world_point: Element) -> Color  {
            let obj_point = obj.get_transform().invert().unwrap().dot(world_point.matrix).unwrap();
            let pattern_point = Element{ matrix: self.get_transform().invert().unwrap().dot(obj_point).unwrap()};
            self.pattern_at(pattern_point)
    
        }
}
impl RingPattern{
    pub fn new(a: Color, b: Color) -> RingPattern{
        RingPattern{a: a, b: b, transform: Matrix::zero(4,4).identity()}
    }
    pub fn base() -> RingPattern{
        RingPattern{a: Color::white(), b: Color::black(), transform: Matrix::zero(4,4).identity()}
    }
    pub fn get_a(&self) -> Color{
        self.clone().a
    } 
    pub fn get_b(&self) -> Color{
        self.clone().b
    } 
}

#[derive(Debug,Clone,PartialEq)]
pub struct GradientPattern{
    a: Color,
    b: Color,
    transform: Matrix,
}
impl Pattern for GradientPattern{

        fn box_me(self) -> Box<dyn Pattern>{
            Box::new(self) as Box<dyn Pattern>
        }
    
        fn get_transform(&self) -> Matrix{
            self.clone().transform
        } 
    
        fn set_transform(&mut self,t: Matrix){
            let t_c = t.clone();
            self.transform = t_c;
        }
        fn pattern_at(&self,p: Element) -> Color {
            let distance = self.get_b() - self.get_a();
            eprintln!("{:?}",distance);
            let fraction = p.x() - p.x().floor();

            self.get_a() + (distance * fraction)
        }
    
        fn pattern_at_shape(&self, obj: & Box<dyn ShapeThings>, world_point: Element) -> Color  {
            let obj_point = obj.get_transform().invert().unwrap().dot(world_point.matrix).unwrap();
            let pattern_point = Element{ matrix: self.get_transform().invert().unwrap().dot(obj_point).unwrap()};
            self.pattern_at(pattern_point)
    
        }
}
impl GradientPattern{
    pub fn new(a: Color, b: Color) -> GradientPattern{
        GradientPattern{a: a, b: b, transform: Matrix::zero(4,4).identity()}
    }
    pub fn base() -> GradientPattern{
        GradientPattern{a: Color::white(), b: Color::black(), transform: Matrix::zero(4,4).identity()}
    }
    pub fn get_a(&self) -> Color{
        self.clone().a
    } 
    pub fn get_b(&self) -> Color{
        self.clone().b
    } 
}
pub trait Pattern: ClonePattern + std::fmt::Debug{
    fn pattern_at(&self,p: Element) -> Color;
    fn pattern_at_shape(&self, obj: & Box<dyn ShapeThings>, world_point: Element) -> Color;
    fn get_transform(&self) -> Matrix;
    fn set_transform(&mut self,t: Matrix);
    fn box_me(self) -> Box<dyn Pattern>;


}

impl Pattern for StripePattern{
    fn box_me(self) -> Box<dyn Pattern>{
        Box::new(self) as Box<dyn Pattern>
    }

    fn get_transform(&self) -> Matrix{
        self.clone().transform
    } 

    fn set_transform(&mut self,t: Matrix){
        let t_c = t.clone();
        self.transform = t_c;
    }
    fn pattern_at(&self,p: Element) -> Color {
        if p.x().floor()%2.0 == 0.0 {
            self.get_a()
        } else{
            self.get_b()
        }
    }

    fn pattern_at_shape(&self, obj: & Box<dyn ShapeThings>, world_point: Element) -> Color  {
        let obj_point = obj.get_transform().invert().unwrap().dot(world_point.matrix).unwrap();
        let pattern_point = Element{ matrix: self.get_transform().invert().unwrap().dot(obj_point).unwrap()};
        self.pattern_at(pattern_point)

    }

}
#[derive(Debug,Clone,PartialEq)]
pub struct TestPattern{
    transform: Matrix,
}
impl Pattern for TestPattern{

        fn box_me(self) -> Box<dyn Pattern>{
            Box::new(self) as Box<dyn Pattern>
        }
    
        fn get_transform(&self) -> Matrix{
            self.clone().transform
        } 
    
        fn set_transform(&mut self,t: Matrix){
            let t_c = t.clone();
            self.transform = t_c;
        }
        fn pattern_at(&self,p: Element) -> Color {
            Color::new(p.x(),p.y(),p.z())
        }
    
        fn pattern_at_shape(&self, obj: & Box<dyn ShapeThings>, world_point: Element) -> Color  {
            let obj_point = obj.get_transform().invert().unwrap().dot(world_point.matrix).unwrap();
            let pattern_point = Element{ matrix: self.get_transform().invert().unwrap().dot(obj_point).unwrap()};
            self.pattern_at(pattern_point)
    
        }
}
impl TestPattern{
    pub fn new() -> TestPattern{
        TestPattern{transform: Matrix::zero(4,4).identity()}
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct StripePattern {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,

}

impl StripePattern {
    pub fn get_a(&self) -> Color{
        self.clone().a
    } 
    pub fn get_b(&self) -> Color{
        self.clone().b
    } 
    
    pub fn new(a: Color, b: Color) -> StripePattern{
        StripePattern{
            a: a,
            b: b,
            transform: Matrix::zero(4,4).identity(),
        }
    }
    pub fn base() -> StripePattern{
        StripePattern::new(Color::white(),Color::black())
    }

    
}







pub trait A {
    fn as_any(&self) -> &dyn Any;
    fn make(&self, h: Vec<f64>) -> Intersections;
    fn equal(&self, other:& Box<dyn ShapeThings>) -> bool;
}


//trait bound here means any static type implement partialeq fit
// impl <S: 'static + PartialEq> A for S{ //need static so box<shapething> is object safe?
//     fn as_any(&self) -> &dyn Any {
//         self
//     }    
//     fn equals_a(&self, other: &dyn A) -> bool {
//         other
//             .as_any()
//             .downcast_ref::<S>()
//             .map_or(false, |a| self == a)
//     }
// }
// to test for equality between trait objects, not my thinking

impl A for Box<dyn ShapeThings>{
    fn as_any(&self) -> &dyn Any {
        self
    }    
    fn make(&self, h: Vec<f64>) -> Intersections{
        let mut list = vec![];
        for (_i,e) in h.iter().enumerate(){
            let here = Intersection{ t: *e , o: self};
            list.push(here);
        }
        Intersections { count: h.len() as u32, h: list }
    }
    fn equal(&self, other: & Box<dyn ShapeThings>) -> bool{
        self.get_material() == other.get_material() && self.get_transform() == other.get_transform()
    }

}





pub trait ShapeThings: CloneFoo + Any + std::fmt::Debug { //we cannot use partialeq here bc it calls on Self, which for box<dyn..> we cannot have
    
    fn intersect(&self,r: &Ray) -> Vec<f64>;
    fn get_transform(&self) -> Matrix;
    fn normal_at(&self, pos: &Element) -> Element;
    fn get_material(&self) -> Material;
    fn set_transform(&mut self,t: Matrix);
    
    fn this_is(self) -> Box<dyn ShapeThings>;

}

// shapethings:clonefoo is blanket implementation, shapethings and clonefoo are "same", a clonefoo can call on shapething's methods and visa versa
pub trait CloneFoo {
    fn clone_box(&self) -> Box<dyn ShapeThings>;
}

impl<T> CloneFoo for T
    where T: 'static + ShapeThings + Clone,
{
    fn clone_box(&self) -> Box<dyn ShapeThings> { //dyn requires at run time to look up which method to call on which type that implements the trait
        Box::new(self.clone())
    }
}
//defining Clone for Box<>
impl Clone for Box<dyn ShapeThings> {
    fn clone(&self) -> Box<dyn ShapeThings> {
        self.clone_box()
    }
}
/*not my workaround
implement Clone trait for box, which is override (calls on the clone box method, which here we can actually clone?; 
    method is defined for shapethings type bound with static and clone)
    //cannot just extend Clone bc Box is dynamic which means the size is not known at compile time, but Clone returns Self (which needs defined size at compile time)
    //Box<> is a trait object, box is the object, shapethings is the trait, we cannot know the size at compile time bc we lose the granularity when putting into box
    //edit: we lose granularity at the type level bc we ONLY know the trait, the type could be any with their own any methods; so now we have to define Clone where
    it does not require the use of returning Self (bc it is not object safe to return Self since the original type is forgotten)
*/
pub trait ClonePattern {
    fn clone_box(&self) -> Box<dyn Pattern>;
}

impl<T> ClonePattern for T
    where T: 'static + Pattern + Clone,
{
    fn clone_box(&self) -> Box<dyn Pattern> { //dyn requires at run time to look up which method to call on which type that implements the trait
        Box::new(self.clone())
    }
}
//defining Clone for Box<>
impl Clone for Box<dyn Pattern> {
    fn clone(&self) -> Box<dyn Pattern> {
        self.clone_box()
    }
}
#[derive(Debug, Clone, PartialEq)]
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
    
    fn intersect(&self,r: &Ray) -> Vec<f64>{

        eprintln!("{:?}", r);
        vec![] 
    }
    fn get_transform(&self) -> Matrix {
        self.clone().transform
    }
    fn normal_at(&self, pos: &Element) -> Element{
        let mut p = pos.clone();
        p.matrix.matrix[3][0] = 0.0;
        p
    }
    fn get_material(&self) -> Material {
        let m_c = self.clone();
        m_c.material
    }
    fn set_transform(&mut self,t: Matrix){
        let t_c = t.clone();
        self.transform = t_c;

    }

    fn this_is(self) -> Box<dyn ShapeThings>
    {
        Box::new(self) as Box<dyn ShapeThings>
    
    }

}



#[derive(Debug,Clone,PartialEq)]
pub struct Plane{
    pub transform: Matrix,
    pub material: Material,
}

impl Plane {
    pub fn test() -> Plane{
        Plane{
            transform: Matrix::zero(4,4).identity(),
            material: Material::new(),
            
        }
    }
}


impl ShapeThings for Plane{

    
    fn intersect(&self,r: &Ray) -> Vec<f64>{
        
        if (r.direction.y()).abs() < EPSILON{
            vec![]
        } else {
            let t = -r.origin.y()/r.direction.y();
            
            vec![t]  //why does it not know plane has shapethings/ q: dyn shapethings == shapethings?
            //Plane is too defined, need to push it to trait level
        }
    }
    fn this_is(self) -> Box<dyn ShapeThings>
    {
        Box::new(self) as Box<dyn ShapeThings>
    
    }
    
    fn get_transform(&self) -> Matrix {
        self.clone().transform
    }
    fn normal_at(&self, _pos: &Element) -> Element{
        vector(0.0,1.0,0.0)
    }
    fn get_material(&self) -> Material {
        let m_c = self.clone();
        m_c.material
    }
    fn set_transform(&mut self,t: Matrix){
        let t_c = t.clone();
        self.transform = t_c;

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
    fn intersect(&self,r: &Ray) -> Vec<f64>{
         //why ref here too?, isnt obj already one?, accessing fields changes this?
        
        //maybe have dot use reference so we dont have to keep repeating clone
        let a = r.clone().direction.dot(r.clone().direction);
        let b = 2.0 * r.clone().direction.dot(r.clone().sphere_to_ray(&self));
        let c = r.clone().sphere_to_ray(&self).dot(r.clone().sphere_to_ray(&self)) - 1.0; 
        //radius is still 1 bc we are scaling the ray, operating in object space
        // eprintln!("t_r: {:?}", t_r); //correct
        // eprintln!("sphere loc: {:?}", obj.loc);
        // eprintln!("sphere to ray: {:?}", t_r.clone().sphere_to_ray(&obj));
        // eprintln!("a: {:?}, \n b: {:?} \n c: {:?} ", a,b,c);
            //a is modulus squared
            //b 
        let discri = b.powi(2) - 4.0 * a * c;
        if discri < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discri.sqrt())/(2.0*a);
            let t2 = (-b + discri.sqrt())/(2.0*a);
            //even if the t is in object space, why should it be different? the direction has been scaled as well so it should cancel out
            //let s1 =  t_r.position(t1).z();
            //let s2 =  t_r.position(t2).z(); //s's are positions of intersections
            //distance away is given by position()
 
            vec![t1,t2]         
            }
        }
    fn get_transform(&self) -> Matrix {
        self.clone().transform
    }

    fn normal_at(&self, pos: &Element) -> Element {
        //using clone to dereference pos (cant just do it bc it is shared), dont need to for obj bc by accessing its field, it already does so -> edit: jk need for obj too
        let obj_0 = self.clone();
        //let object_poi = (obj_0.transform.invert().unwrap()).dot(pos_0.matrix).unwrap(); //poi in object space
        
        let object_nor = pos.clone() - point(0.0,0.0,0.0); //get normal dir of (poi and sphere) in object space, hint: finding normal from 0,0,0 (sphere)
        //let mut world_nor = ((obj_0.transform.invert().unwrap()).transpose()).dot(object_nor.matrix).unwrap(); //
        //object_nor.matrix.matrix[3][0] = 0.0;
        //Element{matrix: object_nor}.normal()
        object_nor
    }
    fn get_material(&self) -> Material {
        let m_c = self.clone();
        m_c.material
    }
    fn set_transform(&mut self,t: Matrix){
        let t_c = t.clone();
        self.transform = t_c;

    }
    fn this_is(self) -> Box<dyn ShapeThings>
    {
        Box::new(self) as Box<dyn ShapeThings>
    
    }
    
}

pub fn normal_at<S: ?Sized>(obj: &S, pos: &Element) -> Element //what does it mean when size is unknown
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
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<Box<dyn Pattern>>,
}

impl PartialEq for Box<dyn Pattern>  {
    fn eq(&self, other: &Self) -> bool {
        self.get_transform() == other.get_transform() 
       
    }
}
impl Material{
    pub fn new() -> Material{
        Material { color: Color::new(1.0,1.0,1.0), ambient: 0.1 , diffuse: 0.9, specular: 0.9, shininess: 200.0 ,pattern: None }
    }
}



pub fn lighting(m: Material, object: &Box<dyn ShapeThings>, light:&PointLight,position: Element,eyev: Element,normalv:Element,in_shadow:bool) -> Color  {
    //let e = Element {matrix: m.color.matrix.dot(light.clone().intensity.matrix).unwrap()};
    let mut color = m.color;
    if m.pattern.is_some(){
        color = m.pattern.unwrap().pattern_at_shape(object, position.clone());
    }
    let light = light.clone();
    let effective_color = color * light.clone().intensity;
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
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color{
    pub fn new(r: f64, g: f64, b: f64) -> Color{
        Color { r: r, g: g, b: b }
    }
    pub fn black() -> Color { Color{r: 0.0, g: 0.0, b: 0.0}}
    pub fn white() -> Color { Color{r: 1.0, g: 1.0, b: 1.0}}
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
        Color { r: (self.r - other.r).clamp(-1.0,1.0),
            g: (self.g - other. g).clamp(-1.0,1.0), 
            b: (self.b - other.b).clamp(-1.0,1.0)  }
    }
}


impl Add for Color{
    type Output = Color;
    fn add(self, other: Color) -> Color{
        Color { r: (self.r + other.r).clamp(-1.0,1.0),
                g: (self.g + other. g).clamp(-1.0,1.0), 
                b: (self.b + other.b).clamp(-1.0,1.0) }
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, other: f64) -> Color{
        Color { r: (self.r * other).clamp(-1.0,1.0),
            g: (self.g * other).clamp(-1.0,1.0), 
            b: (self.b * other).clamp(-1.0,1.0)}
    }
}


impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color{
        Color { r: (self.r * other.r).clamp(-1.0,1.0),
            g: (self.g * other. g).clamp(-1.0,1.0), 
            b: (self.b * other.b).clamp(-1.0,1.0) }
    }
}

#[derive(Clone)]
pub struct World{
    pub objects: Vec<Box<dyn ShapeThings>>,
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
        World { objects: vec![Box::new(s1) as Box<dyn ShapeThings> ,Box::new(s2) as Box<dyn ShapeThings>] , light_source: PointLight::new(point(-10.0,10.0,-10.0), Color::new(1.0,1.0,1.0)) }
    }
////
}
    pub fn intersect_world<'a>(w: &'a World,r:&'a Ray,mut l:  Vec<Intersection<'a>>) -> Intersections<'a>{

        for (_i,j) in w.objects.iter().enumerate(){
          
            let j = &*j;
            let  v = intersect_shape(r,j);
            for (_a,b) in v.h.iter().enumerate(){   //CHECK 
                let hits = b.clone(); //issue now bc intersect takes ownership of j
                l.push(hits);
            }
        }
        l.sort_by(|e1 ,e2| e1.t.partial_cmp(&e2.t).unwrap()); //nm
        let list_l= l.len() as u32;
        let list_s = l.clone();
        Intersections{ 
            count: list_l,
            h: list_s,
        }
    }


#[derive(Clone, Debug)]
pub struct Computations{
    pub t: f64,
    pub object: Box<dyn ShapeThings>,
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
        let object = &*i.o;
        let point =r.position(t);
        let mut normalv = normal_at(&**object, &point);
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


pub fn shade_hit(world: &World, obj: & Box<dyn ShapeThings> , comps: Computations) -> Color{
    let shadowed = is_shadowed(world, &comps.over_point);
    lighting(comps.object.get_material(), obj, &world.light_source, comps.point, comps.eyev, comps.normalv,shadowed)
}

pub fn color_at(w: &World, r: &Ray) -> Color{
    let mut intersections = intersect_world(w,r, vec![]);
    let hit = hit(&mut intersections);

    if hit.is_none() {
        Color::new(0.0,0.0,0.0)
    } else {
        let comp = Computations::prepare_computations(hit.unwrap(),r);
        shade_hit(w,hit.unwrap().o ,comp)


    }

}

pub fn view_transform(from: Element ,to: Element , up: Element) -> Matrix {
    let forward = (to-from.clone()).normal(); 
    let upn = up.normal();
    let left = forward.clone().cross(upn);
    let true_up = left.cross(forward.clone());
    let orientation = Matrix::new(vec![vec![left.x(),left.y(),left.z(),0.0],
                                                vec![true_up.x(),true_up.y(),true_up.z(),0.0],
                                                vec![-forward.x(),-forward.y(),-forward.z(),0.0],
                                                vec![0.0,0.0,0.0,1.0]]);
    orientation.dot(translation(-from.x(),-from.y(),-from.z())).unwrap()

}

#[derive(Debug)]
pub struct Camera{
    pub hsize: u32,
    pub vsize: u32,
    pub field_of_view: f64,
    pub transform: Matrix,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,

}

impl Camera{
    pub fn new(hsize:u32,vsize: u32,
        field_of_view: f64,) -> Camera {
            let half_view = (field_of_view/2.0).tan();
            let aspect = hsize as f64 /vsize as f64;
            let half_width;
            let half_height;
            if aspect >= 1.0 {
                half_width = half_view;
                half_height=half_view/aspect;
            } else{
                half_width = half_view*aspect;
                half_height= half_view;
            }
            let pixel_size = half_width * 2.0 / hsize as f64;
            Camera { hsize: hsize, vsize: vsize, field_of_view: field_of_view, transform: Matrix::zero(4,4).identity(), pixel_size: pixel_size, half_height: half_height, half_width: half_width}
        }
    
    pub fn ray_for_pixel(&self,px:u32,py:u32) -> Ray {
        let xoffset = (px as f64 +  0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset; //f64 has copy trait
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
    let mut intersections = intersect_world(&world,&ray, vec![]); //removed clone from world -> temp value is now not freed?
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