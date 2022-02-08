use std::iter::Inspect;
use std::collections::HashMap;
use std::any::Any;


use crate::{Element, vector, point,matrix,Matrix};
use crate::shapes::{A,Sphere,Shape,ShapeThings};


// Transformations
// pub fn transform(r: &Ray, f: fn(f32,f32,f32) -> Matrix, ) -> Ray {
//     let r_c = r.clone();
//     Ray {
//         origin: r_c.origin.f(f32,f32,f32),
//         direction: r_c.direction.f(f32,f32,f32)
//     }
// }



pub fn hit<'a>(t: &'a mut Intersections) -> Option<&'a Intersection<'a>>{
    //finds the closest non negative number
    //intersections is a vec of intersection {t,o}
    let mut curr = HashMap::new();
    for i in 0..(t.count as usize){  
        curr.insert(i,&t.h[i]);
    }

    let t_h = &mut t.h;
    t_h.sort_by(|e1 ,e2| e1.t.partial_cmp(&e2.t).unwrap());
    let mut this_one = None;
    'check: for i in 0..(t.count as usize) {
        if t_h[i].t > 0.0 {
            
            this_one = Some(&t_h[i]);
            //returning a clone of the initial references
            break 'check; 
        } else {
            continue;   
        }
    } 
    this_one

}
//function gets destroyed, using a cloned value, it gets destroyed. removed clone for t_h and insert mut for input to fix



// pub fn intersect<'a>(r: &Ray, obj: &'a Sphere) -> Intersections<'a>{

//     let a = r.clone().direction.dot(r.clone().direction);
//     let b = 2.0 * r.clone().direction.dot(r.clone().sphere_to_ray());
//     let c = r.clone().sphere_to_ray().dot(r.clone().sphere_to_ray()) - 1.0; 

//     let discri = b.powi(2) - 4.0 * a * c;
//     if discri < 0.0 {
//         Intersections { count: 0,  h: vec![]}
//     } else {
//         let t1 = -b - discri.sqrt()/(2.0*a);
//         let t2 = -b + discri.sqrt()/(2.0*a);
//         let s1 =  r.position(t1).z();
//         let s2 =  r.position(t2).z();
//         //distance away is given by position()
//         let i1 = Intersection::new(t1,&obj);
//         let i2 = Intersection::new(t2,&obj);
//         Intersections { count: 2,  h: vec![i1,i2]}            
//         }
//     }
// pub fn intersect<'a>(r: &Ray, obj: &Sphere) -> Intersections<'a>{
      

//         let t_r = r.clone().transform(&obj.transform.invert().unwrap()); //why ref here too?, isnt obj already one?, accessing fields changes this?
        
//         //maybe have dot use reference so we dont have to keep repeating clone
//         let a = t_r.clone().direction.dot(t_r.clone().direction);
//         let b = 2.0 * t_r.clone().direction.dot(t_r.clone().sphere_to_ray(&obj));
//         let c = t_r.clone().sphere_to_ray(&obj).dot(t_r.clone().sphere_to_ray(&obj)) - 1.0; 
//         //radius is still 1 bc we are scaling the ray, operating in object space
//         // eprintln!("t_r: {:?}", t_r); //correct
//         // eprintln!("sphere loc: {:?}", obj.loc);
//         // eprintln!("sphere to ray: {:?}", t_r.clone().sphere_to_ray(&obj));
//         // eprintln!("a: {:?}, \n b: {:?} \n c: {:?} ", a,b,c);
//             //a is modulus squared
//             //b 
//         let discri = b.powi(2) - 4.0 * a * c;
//         if discri < 0.0 {
//             Intersections { count: 0,  h: vec![]}
//         } else {
//             let t1 = (-b - discri.sqrt())/(2.0*a);
//             let t2 = (-b + discri.sqrt())/(2.0*a);
//             //even if the t is in object space, why should it be different? the direction has been scaled as well so it should cancel out
//             let s1 =  t_r.position(t1).z();
//             let s2 =  t_r.position(t2).z(); //s's are positions of intersections
//             //distance away is given by position()
//             let i1 = Intersection::new(t1,&obj);
//             let i2 = Intersection::new(t2,&obj);
//             Intersections { count: 2,  h: vec![i1,i2]}            
//             }
//         }
    
pub fn intersect_shape<'a>(r: &Ray, obj: &'a Box<dyn ShapeThings>) -> Intersections<'a>{
    
    let local_ray = r.transform(&obj.get_transform().invert().unwrap());
    obj.intersect(&local_ray) //use lifetime bc using reference to get around the cannot return something owned by the function constraint
    
}



#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Element,
    pub direction: Element,
}



//pub fn intersection(t:Vec<<f32>>, obj: Element);
#[derive(Debug,Clone)]
pub struct Intersection<'a> {
    pub t : f32,
    pub o : &'a Box<dyn ShapeThings>, //box bc nothing holds traits

}

impl<'a>PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.o.get_transform() == other.o.get_transform()
        &&  self.o.get_material() == other.o.get_material()
    }
    // the object should be the same when this is called anyways, only t would matter (assuming here), if not
    //we are saying that if a shape's transform and material are the same then it is the same 
    // this is saying the intersection ITSELF is the same in the viewpoint of the ray (the ray doesnt know if it intersects another shape made of the same thing)
    // not the shape is the same
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32,o: &'a Box<dyn ShapeThings>) -> Intersection<'a>{

        Intersection{
            t: t,
            o: o,
        }
    }
}
#[derive( Debug,Clone)]
pub struct Intersections<'a>{
    pub count: u32,
    pub h: Vec<Intersection<'a>>,

}

impl<'a> Intersections<'a>{
    pub fn new(i: Vec<Intersection>) -> Intersections{
        Intersections {
            count: i.len() as u32,
            h: i

        }
    }
}

impl Ray{
    pub fn new(p:Element,d:Element) -> Ray{
        Ray{
            origin: p,
            direction: d,
        }
    }
    pub fn ori(self) -> Element{
        self.origin
    }

    pub fn dir(self) -> Element{
        self.direction
    }

    pub fn position(&self, t: f32) -> Element{
        let temp = self.clone();
        let temp2 = self.clone();
        temp.ori() + temp2.dir() * t
    }

    pub fn sphere_to_ray(&self, s: &Sphere) -> Element{
        let temp_r = self.clone();
        let temp_s = s.clone();
        temp_r.origin - temp_s.loc
        

    }

    pub fn transform(&self, t_m: &Matrix) -> Ray  {
        let r_c = self.clone();
            Ray {
                origin: Element {matrix: t_m.dot(r_c.origin.matrix).unwrap()},
                direction: Element {matrix: t_m.dot(r_c.direction.matrix).unwrap()}
            }
        }
    
        
}

