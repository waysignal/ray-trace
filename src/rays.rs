use std::iter::Inspect;
use std::collections::HashMap;

use crate::{Element, vector, point};
use crate::Matrix;


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

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Element,
    pub direction: Element,
}

pub fn intersect<'a>(r: &Ray, obj: &'a Element) -> Intersections<'a>{
    let a = r.clone().direction.dot(r.clone().direction);
    let b = 2.0 * r.clone().direction.dot(r.clone().sphere_to_ray());
    let c = r.clone().sphere_to_ray().dot(r.clone().sphere_to_ray()) - 1.0; 

    let discri = b.powi(2) - 4.0 * a * c;
    if discri < 0.0 {
        Intersections { count: 0,  h: vec![]}
    } else {
        let t1 = -b - discri.sqrt()/(2.0*a);
        let t2 = -b + discri.sqrt()/(2.0*a);
        let s1 =  r.position(t1).z();
        let s2 =  r.position(t2).z();
        //distance away is given by position()
        let i1 = Intersection::new(t1,&obj);
        let i2 = Intersection::new(t2,&obj);
        Intersections { count: 2,  h: vec![i1,i2]}            
        }
    }


//pub fn intersection(t:Vec<<f32>>, obj: Element);
#[derive(Debug, Clone,PartialEq)]
pub struct Intersection<'a> {
    pub t : f32,
    pub o : &'a Element,

}

impl<'a> Intersection<'a> {
    pub fn new(t: f32,o: &'a Element) -> Intersection<'a>{

        Intersection{
            t: t,
            o: o,
        }
    }
}
#[derive(Debug, Clone)]
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

    pub fn sphere_to_ray(&self) -> Element{
        let temp = self.clone();
        temp.origin - point(0.0,0.0,0.0)
        

    }
}
