
use std::collections::HashMap;



use crate::{Element,Matrix,point};
use crate::shapes::{*};
use core::cell::RefCell;
use std::rc::Rc;

// Transformations
// pub fn transform(r: &Ray, f: fn(f64,f64,f64) -> Matrix, ) -> Ray {
//     let r_c = r.clone();
//     Ray {
//         origin: r_c.origin.f(f64,f64,f64),
//         direction: r_c.direction.f(f64,f64,f64)
//     }
// }



pub fn hit(t: & mut Intersections) -> Option<&Intersection>{
    //finds the closest non negative number
    //intersections is a vec of intersection {t,o}
    let mut curr = HashMap::new();
    for i in 0..(t.count as usize){  
        curr.insert(i,&t.h[i]);
    }

    let t_h = &mut t.h;
    t_h.sort_by(|e1 ,e2| e1.t.partial_cmp(&e2.t).unwrap()); //nm
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
    
pub fn intersect_shape(r: & Ray, obj: Box<dyn ShapeThings>) -> Intersections{
    
    let local_ray = r.transform(&obj.get_transform().invert().unwrap());
    
    //let parent = &**g;
    let parent_members = obj.get_members().clone();

    
    if obj.get_kind() == Shapes::Group && !obj.bounding_box().intersect(r){
        //eprintln!("rays. no bbox hit");
        return Intersections::empty();
    }
    else if obj.get_kind() == Shapes::Group {
        //eprintln!("rays. group ");
        let obj = obj.clone();
        let members = take_members_out(&Rc::new(RefCell::new(obj)));
        let mut results = Intersections::empty();
        
        for (_i,e) in members.iter().enumerate(){
            //let e = e.clone();
            // let e = e.clone();
            
            // let mut j = e.into_inner();
            // let matrix = obj.get_transform().dot(j.get_transform()).unwrap();
            // j.set_transform(matrix);
            let x = e.clone();
            let xs = e.intersect(&local_ray);
            if xs.is_empty() { 
                continue;
            }
            let is = make(Rc::new(x),xs);
            //list.push(j);
            results = results.adder(is);
        }
        return results;
        
        //return list
        
        
    }
    //eprintln!("taking33 {:?}",results);
    let xs = obj.intersect(&local_ray);
    //eprintln!("rays. xs {:?}",xs);
    make(Rc::new(obj),xs)

    
}
// pub fn intersect_shape_group<'a>(r: &'a Ray, obj: &'a Box<dyn ShapeThings>, members: Vec<Box<dyn ShapeThings>>,mut list: Vec<Intersections>) -> Intersections<'a>{
    
//     let local_ray = r.transform(&obj.get_transform().invert().unwrap());
    
//     //let parent = &**g;
//     //let parent_members = obj.get_members().clone();

    
//     if obj.get_kind() == Shapes::Group && obj.intersect(r).is_empty(){
//         return Intersections::empty();
//     }
//     else if obj.get_kind() == Shapes::Group {
//         let obj = obj.clone();
//         let mut results = Intersections::empty();
        
//         for (_i,e) in members.iter().enumerate(){
//             //let e = e.clone();
//             // let e = e.clone();
            
//             // let mut j = e.into_inner();
//             // let matrix = obj.get_transform().dot(j.get_transform()).unwrap();
//             // j.set_transform(matrix);
            
//             let xs = e.intersect(&local_ray);
//             let is = e.make(xs);
//             //list.push(j);
//             list.push(is);
//         }
//         return results;
        
//         //return list
        
        
//     }
//     //eprintln!("taking33 {:?}",results);
//     let xs = obj.intersect(&local_ray);
//     obj.make(xs)

    
// }
// let list =  take_members_out_2(obj,r, vec![]);
//         for (_i, e) in list.iter().enumerate(){
//             //let e = e.clone();
//             let xs = e.intersect(&local_ray);
//             let mut it = vec![];
//             for i in xs{
//                 it.push(Intersection{t: i, o: e});
//             }
//             let rr = Intersections::new(it);

//             //let r = e.make(xs);
//             results = results.adder(rr);
//             eprintln!("results");
//         }
//         //return results;v

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Element,
    pub direction: Element,
}



//pub fn intersection(t:Vec<<f64>>, obj: Element);
#[derive(Debug,Clone)]
pub struct Intersection {
    pub t : f64,
    pub o : Rc<Box<dyn ShapeThings>>, //box bc nothing holds traits
 

}




impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.o.get_transform() == other.o.get_transform() && self.o.get_material() == other.o.get_material()
        && self.o.get_kind() == other.o.get_kind() 
       
    }
    // the object should be the same when this is called anyways, only t would matter (assuming here), if not
    //we are saying that if a shape's transform and material are the same then it is the same 
    // this is saying the intersection ITSELF is the same in the viewpoint of the ray (the ray doesnt know if it intersects another shape made of the same thing)
    // not the shape is the same
}

impl Intersection {
    //pub fn new(t: f64,o: &'a Box<dyn ShapeThings>) -> Intersection<'a>{
    pub fn new(t: f64, o : Rc<Box<dyn ShapeThings>>) -> Intersection{
        Intersection{
            t: t,
            o: o,

        }
    }
}
#[derive( Debug,Clone)]
pub struct Intersections{
    pub count: u8,
    pub h: Vec<Intersection>,

}

impl Intersections{
    pub fn new(i: Vec<Intersection>) -> Intersections{
        Intersections {
            count: i.len() as u8,
            h: i

        }
    }
    pub fn empty() -> Intersections{
        Intersections { count: 0, h: vec![] }
    }
    pub fn adder(mut self,mut other: Intersections) -> Intersections{
        self.h.append(&mut other.h);
        
        Intersections { count: self.count + other.count, h: self.h }
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

    pub fn position(&self, t: f64) -> Element{
        let temp = self.clone();
        let temp2 = self.clone();
        temp.ori() + temp2.dir() * t
    }

    pub fn sphere_to_ray(&self, s: &Sphere) -> Element{
        let temp_r = self.clone();
        let temp_s = s.clone();
        temp_r.origin - point(0.0,0.0,0.0)
        


    }

    pub fn transform(&self, t_m: &Matrix) -> Ray  {
        let r_c = self.clone();
            Ray {
                origin: Element {matrix: t_m.dot(r_c.origin.matrix).unwrap()},
                direction: Element {matrix: t_m.dot(r_c.direction.matrix).unwrap()}
            }
        }
    
        
}

