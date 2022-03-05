
#[cfg(test)]
pub mod tests{
    use std::borrow::Borrow;
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::shapes::{*};
    use crate::Canvas;
    use crate::matrix::matrix::*;
    use crate::rays::{Ray, hit,  Intersections,Intersection,intersect_shape};
    use crate::{Element,Matrix, vector, point};
    use std::{mem, collections::HashMap};
    //use crate::Ray;
    use std::{f64::consts::PI,};
    use std::rc::Weak;



// //     #[test]
// //     fn ispoint(){
// //         let tester = point(1.0,2.0,3.0);
// //         assert_eq!(1.0,tester.grabtype());
// //     }

// //     #[test]
// //     fn isvector(){
// //         let tester = vector(1.0,2.0,3.0);
// //         assert_eq!(0.0,tester.grabtype());
// //     }

// //     #[test]
// //     fn newvector(){
// //         let nv = vector(1.0,2.0,3.0);
// //         eprintln!("{:?} ",  nv)
// //     }

// //     #[test]
// //     fn clock(){
// //         let mut clock = Canvas::new(200,200);
// //         let r = 3.0/8.0 * 200.0;
// //         let mut points = vec![];
// //         for i in 0..12{
// //             let mut p = Element { matrix: rotate_z(i as f64 * PI/6.0).dot(point(1.0,0.0,0.0).matrix).unwrap()};
// //             //println!("initial {:?}" , p);
// //             p = p * (r);
// //             //println!("times radius {:?}" , p);
// //             p = Element {matrix:  translation(100.0,100.0,0.0).dot(p.matrix).unwrap()};
// //             println!("final {:?}" , p);
// //             points.push(p)
// //         }

// //         for (_x, y)  in points.iter().enumerate(){
// //             clock.color(y.x() as usize,200 - (y.y() as usize),Color::new(1.0,1.0,1.0))
// //         }

// //         //clock.draw()
// //     }

// //     #[test]
// //     fn rays_feature(){
// //         let p = point(2.0,3.0,4.0);
// //         let d = vector(1.0,0.0,0.0);
// //         let ray = Ray::new(p,d);
// //         let pos = ray.position(0.0);
// //         let pos1 = ray.position(1.0);
// //         let pos2 = ray.position(-1.0);
// //         let pos3 = ray.position(2.5);
// //         assert_eq!(true,pos.matrix.equal(point(3.0,3.0,4.0).matrix));
// //         assert_eq!(true,pos1.matrix.equal(point(3.0,3.0,4.0).matrix));
// //         assert_eq!(true,pos2.matrix.equal(point(1.0,3.0,4.0).matrix));
// //         assert_eq!(true,pos3.matrix.equal(point(4.5,3.0,4.0).matrix));
// //     }

//     // #[test]
//     // //returning t instead of intersect records
//     // fn spheres_feature(){
//     //     let r1 = Ray::new(point(0.0,0.0,-5.0), vector(0.0,0.0,1.0));
//     //     let s1 = point(0.0,0.0,0.0,);
//     //     assert_eq!(Sphere_Hits{count:2, dist: [4.0,6.0]},intersect(&r1,s1.clone()));

//     //     let r2 = Ray::new(point(0.0,1.0,-5.0), vector(0.0,0.0,1.0));
//     //     let s2 = s1.clone();
//     //     assert_eq!(Sphere_Hits{count:2, dist: [5.0,5.0]},intersect(&r2,s2));

//     //     let r3 = Ray::new(point(0.0,2.0,-5.0), vector(0.0,0.0,1.0));
//     //     let s3 = s1.clone();
//     //     assert_eq!(Sphere_Hits{count:0, dist: [0.0,0.0]},intersect(&r3,s3));

//     //     let r4 = Ray::new(point(0.0,0.0,0.0), vector(0.0,0.0,1.0));
//     //     let s4 = s1.clone();
//     //     assert_eq!(Sphere_Hits{count:2, dist: [-1.0,1.0]},intersect(&r4,s4));

//     //     let r5 = Ray::new(point(0.0,0.0,5.0), vector(0.0,0.0,1.0));
//     //     let s5 = s1.clone();
//     //     assert_eq!(Sphere_Hits{count:2, dist: [-6.0,-4.0]},intersect(&r5,s5));
//     // }

//     #[test]
//     fn intersections_feature(){
//         let s = Sphere::new();
//         let s1 = s.this_is();
//         let i1 = Intersection::new(3.5,&s1);
//         assert_eq!(3.5, i1.t);

//         let placehold = i1.o;
//         assert!(&s1.equal( placehold));

//         let s = Sphere::new();
//         let i1 = Intersection::new(1.0,&s1);
//         let i2 = Intersection::new(2.0,&s1);
//         let group = Intersections::new(vec![i1,i2]);
//         assert_eq!(2, group.count);
//         assert_eq!(1.0, group.h[0].t);
//         assert_eq!(2.0, group.h[1].t);
//     }
//     #[test]
//     fn sphere_features(){
//         let r1 = Ray::new(point(0.0,0.0,5.0), vector(0.0,0.0,1.0));
//         let mut s1 = Sphere::new();
//         let xs = s1.intersect(&r1);
//         eprintln!("{:?}",xs);
//         assert_eq!(2,xs.len());
//         assert_eq!(-6.0,xs[0]);
//         assert_eq!(-4.0,xs[1]);
//     }
//     #[test]
//     fn sphere_features_2(){
//         let r1 = Ray::new(point(0.0,0.0,-5.0), vector(0.0,0.0,1.0));
//         let mut s1 = Sphere::new();
//         let xs = s1.intersect(&r1);
//         assert_eq!(2,xs.len());
//         let s1 = s1.this_is();
//         let xs = s1.make(xs);
//         assert!(s1.equal(xs.h[0].o));
//         assert!(s1.equal(xs.h[1].o));
//     }

//     #[test]
//     fn intersection_feature_hits(){
//         let s = Sphere::new();
//         let s_b = s.this_is();
//         let i1 = Intersection::new(1.0,&s_b);
//         let i2 = Intersection::new(2.0,&s_b);
//         let mut xs = Intersections::new(vec![i2,i1]);
//         let xs_clone = xs.clone();
//         assert_eq!(&xs_clone.h[1],hit(&mut xs).unwrap());

//         let i1 = Intersection::new(-1.0,&s_b);
//         let i2 = Intersection::new(1.0,&s_b);
//         let mut xs = Intersections::new(vec![i2,i1]);
//         let xs_clone = xs.clone();
//         assert_eq!(&xs_clone.h[0],hit(&mut xs).unwrap());

//         let i1 = Intersection::new(-2.0,&s_b);
//         let i2 = Intersection::new(-1.0,&s_b);
//         let mut xs = Intersections::new(vec![i2,i1]);
//         assert_eq!(None,hit(&mut xs));

//         let i1 = Intersection::new(5.0,&s_b);
//         let i2 = Intersection::new(7.0,&s_b);
//         let i3 = Intersection::new(-3.0,&s_b);
//         let i4 = Intersection::new(2.0,&s_b);

//         let mut xs = Intersections::new(vec![i1,i2,i3,i4]);
//         //realizing dont need clone if just put a reference to a new intersection
//         assert_eq!(&Intersection::new(2.0,&s_b),hit(&mut xs).unwrap());

//     }

//     #[test]
//     fn ray_feature(){
//         let r = Ray::new(point(1.0,2.0,3.0), vector(0.0,1.0,0.0));
//         let m = translation(3.0, 4.0, 5.0);
//         let r2 = r.transform(&m);
//         assert_eq!(point(4.0,6.0,8.0),r2.origin);
//         assert_eq!(vector(0.0,1.0,0.0),r2.direction);

//         let m = scale(2.0,3.0,4.0);
//         let mut r2 = r.transform(&m);

//         //is it better to keep referencing an object or destory and remake each time?
//         assert_eq!(point(2.0,6.0,12.0),r2.origin);
//         assert_eq!(vector(0.0,3.0,0.0,),r2.direction);
//     }

//     #[test]
//     fn sphere_feature_3(){
//         let mut s = Sphere::new();

//         let t = translation(2.0,3.0,4.0);
//         s.set_transform(&t);
//         assert_eq!(t,s.transform);
//         //we can have set_tranform use a mut ref for self, it will not take ownership and changes the fields

//         let r = Ray::new(point(0.0,0.0,-5.0),vector(0.0,0.0,1.0));
//         let mut s = Sphere::new();

//         s.set_transform(&scale(2.0,2.0,2.0)); // who is the owner of scale? -> the function set_tranform
//         let xs = s.intersect(&r);
//             //needs to be a transformation back -> edit: no there doesnt, t should be correct regardless
//         assert_eq!(scale(2.0,2.0,2.0),s.transform);
//         assert_eq!(2,xs.len());
//         let s = s.this_is();
//         let xs = intersect_shape(&r,&s);
     
//         assert_eq!(7.0,xs.h[1].t);
//         assert_eq!(3.0,xs.h[0].t);

//         let mut s = Sphere::new();;

//         let t = translation(5.0,0.0,0.0);
//         s.set_transform(&t);
//         let s = s.this_is();
//         let xs = intersect_shape(&r,&s);

//         assert_eq!(0,xs.count);
//     }

// //     #[test]
// //     fn first_draw_sphere() {
// //         let wall_z = 10.0;
// //         let wall_size = 7.0;
// //         let canvas_pixels = 100.0;
// //         let pixel_size = wall_size/canvas_pixels;
// //         let half = wall_size/2.0;
// //         let mut new = Canvas::new(100,100);
// //         let mut s = Sphere{loc: point(0.0,0.0,0.0),
// //             transform: Matrix::zero(4,4).identity(),material: Material::new()};
// //         //let mut ray = Ray::new(point(0.0,0.0,-5.0),vector(0.0,0.0,1.0));


// //         let t = shearing(1.0,0.0,0.0,0.0,0.0,0.0).dot(scale(0.5,1.0,1.0)).unwrap();
// //         s.set_transform(&t);

// //         for y in 0..100 {
// //             let world_y = half - pixel_size * (y as f64);

// //             for x in 0..100 {
// //                 let world_x = -half + pixel_size * (x as f64);
// //                 let position = point(world_x, world_y, wall_z);

// //                 let r = Ray::new(point(0.0,0.0,-5.0), (position - point(0.0,0.0,-5.0).normal()));
// //                 let xs = s.intersect(&r);
// //                 if xs.count > 0 {
// //                     new.color(x,y,Color::new(1.0,0.0,0.0));
// //                 }
// //             }

// //         }
// //         //new.draw();
// //     }
//     #[test]
//     fn sphere_feature_4_78(){
//         let mut s = Sphere::new();;

//         let n = normal_at(&s,&point(1.0,0.0,0.0));
//         assert_eq!(vector(1.0,0.0,0.0),n);

//         let n = normal_at(&s,&point(0.0,1.0,0.0));
//         assert_eq!(vector(0.0,1.0,0.0),n);

//         let n = normal_at(&s,&point(0.0,0.0,1.0));
//         assert_eq!(vector(0.0,0.0,1.0),n);

//         let n = normal_at(&s,&point(3.0_f64.sqrt()/3.0_f64,3.0_f64.sqrt()/3.0,3.0_f64.sqrt()/3.0));
//         assert!(vector(3.0_f64.sqrt()/3.0_f64,3.0_f64.sqrt()/3.0,3.0_f64.sqrt()/3.0).matrix.equal(n.clone().matrix)); //make equal same for matrix and element? ; trait but inputs are different, unless we can use self for input, but dynamic?

//         assert!(n.clone().matrix.equal(n.clone().normal().matrix));
//     }
//     #[test]
//     fn sphere_feature_5_80(){
//         let mut s = Sphere::new();
//         s.set_transform(&translation(0.0,1.0,0.0));
//         let n = normal_at( &s,&point(0.0,1.70711,-0.70711));
//         //original normal_at before refactor
//         assert!(vector(0.0,0.70711,-0.70711).matrix.equal(n.matrix));

//         s.set_transform(&scale(1.0,0.5,1.0).dot(rotate_z(PI/5.0)).unwrap());
//         let n = normal_at(& s,&point(0.0,2.0_f64.sqrt()/2.0,-2.0_f64.sqrt()/2.0));
//         eprintln!("{:?}", n);
//         assert!(vector(0.0,0.97014,-0.24254).matrix.equal(n.matrix));

//     }

//     #[test]
//     fn tuples_features_83(){
//         let v = vector(0.0,-1.0,0.0);
//         let n = vector(2.0_f64.sqrt()/2.0,2.0_f64.sqrt()/2.0,0.0);
//         let r = reflect(&v,&n);
//         assert!(vector(1.0,0.0,0.0).matrix.equal(r.matrix));

//         let v = vector(1.0,-1.0,0.0);
//         let n = vector(0.0,1.0,0.0);
//         let r = reflect(&v,&n);
//         assert!(vector(1.0,1.0,0.0).matrix.equal(r.matrix));
//     }
//     #[test]
//     fn lights_feature_84(){
//         let pl = PointLight::new(point(0.0,0.0,0.0), Color::new(1.0,1.0,1.0));
//         assert_eq!(pl.position, point(0.0,0.0,0.0));
//         assert_eq!(pl.intensity, Color::new(1.0,1.0,1.0));

//         let m = Material::new();
//         eprintln!("{:?}",m);

//         let mut s = Sphere::new();
//         assert_eq!(Material::new(), s.material);

//         let mut s = Sphere::new();
//         let mut m = Material::new();
//         m.ambient = 1.0;
//         let m_1 = m.clone();
//         s.material = m;
//         assert_eq!(s.material, m_1);

//     }
//     #[test]
//     fn material(){
//         let mut m = Material::new();
//         let pos = point(0.0,0.0,0.0);

//         let eyev = vector(0.0,0.0,-1.0);
//         let normalv = vector(0.0,0.0,-1.0);
//         let light = PointLight::new(point(0.0,0.0,-10.0), Color::new(1.0,1.0,1.0));

//         let result = lighting(m,&Shape::test().this_is(),&light,pos,eyev,normalv,false);
//         //assert_eq!(Color::new(1.9,1.9,1.9),result); //implemented clamp for colors
//         assert_eq!(Color::new(1.0,1.0,1.0),result);

//         let mut m = Material::new();
//         let pos = point(0.0,0.0,0.0);
//         let eyev = vector(0.0,2.0_f64.sqrt()/2.0,-2.0_f64.sqrt()/2.0);
//         let normalv = vector(0.0,0.0,-1.0);
//         let light = PointLight::new(point(0.0,0.0,-10.0), Color::new(1.0,1.0,1.0));

//         let result = lighting(m,&Shape::test().this_is(),&light,pos,eyev,normalv,false);
//         assert_eq!(Color::new(1.0,1.0,1.0),result);

//         let mut m = Material::new();
//         let pos = point(0.0,0.0,0.0);
//         let eyev = vector(0.0,0.0,-1.0);
//         let normalv = vector(0.0,0.0,-1.0);
//         let light = PointLight::new(point(0.0,10.0,-10.0), Color::new(1.0,1.0,1.0));

//         let result = lighting(m,&Shape::test().this_is(),&light,pos,eyev,normalv,false);
//         //assert_eq!(Color::new(0.7364,0.7364,0.7364),result); //close

//         let mut m = Material::new();
//         let pos = point(0.0,0.0,0.0);
//         let eyev = vector(0.0,-2.0_f64.sqrt()/2.0,-2.0_f64.sqrt()/2.0);
//         let normalv = vector(0.0,0.0,-1.0);
//         let light = PointLight::new(point(0.0,10.0,-10.0), Color::new(1.0,1.0,1.0));

//         let result = lighting(m,&Shape::test().this_is(),&light,pos,eyev,normalv,false);
//         //assert_eq!(Color::new(1.6364,1.6364,1.6364),result); //close


//         let mut m = Material::new();
//         let pos = point(0.0,0.0,0.0);
//         let eyev = vector(0.0,0.0,-1.0);
//         let normalv = vector(0.0,0.0,-1.0);
//         let light = PointLight::new(point(0.0,0.0,10.0), Color::new(1.0,1.0,1.0));

//         let result = lighting(m,&Shape::test().this_is(),&light,pos,eyev,normalv,false);
//         assert_eq!(Color::new(0.1,0.1,0.1),result); //close

//     }
//     // #[test]
//     // fn second_draw_sphere() {
//     //     let wall_z = 10.0;
//     //     let wall_size = 7.0;
//     //     let canvas_pixels = 100.0;
//     //     let pixel_size = wall_size/canvas_pixels;
//     //     let half = wall_size/2.0;

//     //     let mut new = Canvas::new(100,100);
//     //     let mut s = Sphere::new();
        
//     //     s.material.color = Color::new(1.0,0.2,1.0);
        

//     //     let light = PointLight::new(point(-10.0,10.0,-10.0), Color::new(1.0,1.0,1.0));

//     //     // let t = shearing(1.0,0.0,0.0,0.0,0.0,0.0).dot(scale(0.5,1.0,1.0)).unwrap();
//     //     // s.set_transform(&t);
//     //     let s1 = s.clone().this_is();
//     //     for y in 0..100 {
//     //         let world_y = half - pixel_size * (y as f64);

//     //         for x in 0..100 {
//     //             let world_x = -half + pixel_size * (x as f64);
//     //             let position = point(world_x, world_y, wall_z);

//     //             let r = Ray::new(point(0.0,0.0,-5.0), ((position - point(0.0,0.0,-5.0)).normal()));
//     //             let mut xs = s.clone().intersect(&r);
//     //             if xs.len() > 0 {
                    
//     //                 let mut xs = s1.make(xs);
//     //                 let int = hit(&mut xs).unwrap();
//     //                 let int_s = int.clone().o.clone();      /// BYPASSING THE SHARED REFERENCES
//     //                 let point = r.position(int.t);
                    
//     //                 let normal = s.normal_at(&point);
//     //                 let eye = -r.dir();
//     //                 let color = lighting(int_s.get_material(), &light, point, eye, normal,false);
//     //                 new.color(x,y,color);
//     //             }
//     //         }

//     //     }
//     //     //new.draw();
//     // }


//     #[test]
//     fn world_feature_92(){
//         let w = World::new();
//         let r = Ray::new(point(0.0,0.0,-5.0), vector(0.0,0.0,1.0));
//         let xs = intersect_world(&w,&r,vec![]);
//         let w = World::new();
//         assert_eq!(4,xs.count);
//         assert_eq!(4.0,xs.h[0].t);
//         assert_eq!(4.5,xs.h[1].t);
//         assert_eq!(5.5,xs.h[2].t);
//         assert_eq!(6.0,xs.h[3].t);


//     }
//     #[test]
//     fn intersections_feature_p93(){
//         let r = Ray::new(point(0.0,0.0,-5.0), vector(0.0,0.0,1.0));
//         let s = Sphere::new();
//         let i = Intersection { t: 4.0, o: &s.this_is()};

//         let comps = Computations::prepare_computations(&i.clone(),&r, Intersections::empty());
//         assert_eq!(i.clone().t, comps.t);
//          //.as_any().downcast_ref::<dyn ShapeThings>().unwrap());
//         let test = i.clone().o.as_any().downcast_ref::<Sphere>() ; // lose detail when casting as shapethings
//         //eprintln!("{:?}", test);

//         assert_eq!(i.clone().o.get_material(), comps.object.get_material());
//         assert_eq!(i.clone().o.get_transform(), comps.object.get_transform());
//         assert_eq!(point(0.0,0.0,-1.0), comps.point);
//         assert_eq!(vector(0.0,0.0,-1.0), comps.eyev);
//         assert_eq!(vector(0.0,0.0,-1.0), comps.normalv);

//         let r = Ray::new(point(0.0,0.0,0.0), vector(0.0,0.0,1.0));
//         let s = Sphere::new();
//         let i = Intersection { t: 1.0, o: &(Box::new(s) as Box<dyn ShapeThings>)};

//         let comps = Computations::prepare_computations(&i.clone(),&r,Intersections::empty());
//         assert_eq!(i.clone().t, comps.t);
//         assert_eq!(true, comps.inside);
//         assert_eq!(point(0.0,0.0,1.0), comps.point);
//         assert_eq!(vector(0.0,0.0,-1.0), comps.eyev);
//         assert_eq!(vector(0.0,0.0,-1.0), comps.normalv);
//     }
//     #[test]
//     fn world_feature_95(){
//         let w = World::new();
//         let r = Ray::new(point(0.0,0.0,-5.0), vector(0.0,0.0,1.0));
//         let shape = &w.objects[0];
//         let i = Intersection { t: 4.0, o: &shape};
//         let comps = Computations::prepare_computations(&i, &r,Intersections::empty());
//         let c = shade_hit(&w,comps, REMAIN);
//         //assert_eq!(Color::new(0.38066,0.47583,0.2855), c); // yes
//         let mut w = World::new();
//         w.light_source = PointLight::new(point(0.0,0.25,0.0), Color::new(1.0,1.0,1.0));

//         let r = Ray::new(point(0.0,0.0,0.0), vector(0.0,0.0,1.0));
//         let shape = &w.objects[1];
//         let i = Intersection { t: 0.5, o: &shape};
//         let comps = Computations::prepare_computations(&i, &r, Intersections::empty());
//         let c = shade_hit(&w,comps, REMAIN);
//         //assert_eq!(Color::new(0.90498,0.90498,0.90498), c); //yes
//     }
//     #[test]
//     fn world_feature_96(){
//         let w = World::new();
//         let r =Ray::new(point(0.0,0.0,-5.0), vector(0.0,1.0,0.0));
//         let c = color_at(&w, &r, REMAIN);
//         assert!(c.equal(Color::new(0.0,0.0,0.0)));

//         let w = World::new();
//         let r =Ray::new(point(0.0,0.0,-5.0), vector(0.0,0.0,1.0));
//         let c = color_at(&w, &r, REMAIN);
//         assert!(c.equal(Color::new(0.38066, 0.47583, 0.2855)));

//         let mut w = World::new();
//         let mut m_new = w.objects[0].get_material();
//         m_new.ambient = 1.0;
//         w.objects[0].set_material(m_new.clone());
//         w.objects[1].set_material(m_new);
//         let r = Ray::new(point(0.0,0.0,0.75), vector(0.0,0.0,-1.0));
//         let c = color_at(&w, &r, 0);
//         let this = &w.objects[1].get_material().color;
//         assert!(c.equal(this.clone()));
//     }

//     #[test]
//     fn transformation_feature(){
//         let from = point(0.0,0.0,0.0);
//         let to = point(0.0,0.0,-1.0);
//         let up = vector(0.0,1.0,0.0);

//         let t = view_transform(from,to,up);
//         assert_eq!(Matrix::zero(4,4).identity(),t);

//         let from = point(0.0,0.0,0.0);
//         let to = point(0.0,0.0,1.0);
//         let up = vector(0.0,1.0,0.0);

//         let t = view_transform(from,to,up);
//         assert_eq!(scale(-1.0,1.0,-1.0),t);

//         let from = point(0.0,0.0,8.0);
//         let to = point(0.0,0.0,0.0);
//         let up = vector(0.0,1.0,0.0);

//         let t = view_transform(from,to,up);
//         assert_eq!(translation(0.0,0.0,-8.0),t);


//         let from = point(1.0,3.0,2.0);
//         let to = point(4.0,-2.0,8.0);
//         let up = vector(1.0,1.0,0.0);

//         let t = view_transform(from,to,up);
//         eprintln!("{:?}", t)
//     }
//     #[test]
//     fn camera_feature_101(){
//         let c = Camera::new(125,200,PI/2.0);
//         eprintln!("{:?}", c);
//     }
//     #[test]
//     fn camera_feature_103(){
//         let c = Camera::new(201,101,PI/2.0);
//         let r = c.ray_for_pixel(100, 50);
//         assert_eq!(point(0.0,0.0,0.0), r.origin);
//         assert!(vector(0.0,0.0,-1.0).matrix.equal(r.direction.matrix));

//         let c = Camera::new(201,101,PI/2.0);
//         let r = c.ray_for_pixel(0, 0);
//         assert_eq!(point(0.0,0.0,0.0), r.origin);
//         assert!(vector(0.66519,0.33259,-0.66851).matrix.equal(r.direction.matrix));

//         let mut c = Camera::new(201,101,PI/2.0);
//         c.transform = rotate_y(PI/4.0).dot(translation(0.0,-2.0,5.0)).unwrap();
//         let r = c.ray_for_pixel(100, 50);
//         assert!(point(0.0,2.0,-5.0).matrix.equal(r.origin.matrix));
//         assert!(vector(2.0_f64.sqrt()/2.0,0.0,-2.0_f64.sqrt()/2.0).matrix.equal(r.direction.matrix));
//     }

//     #[test]
//     fn camera_feature_104(){
//         let w = World::new();
//         let mut c = Camera::new(11,11,PI/2.0);
//         let from = point(0.0,0.0,-5.0);
//         let to = point(0.0,0.0,0.0);
//         let up = vector(0.0,1.0,0.0);
//         c.transform = view_transform(from, to, up);

//         let image = render(c, w);
//         //assert_eq!(image.pixels[5][5], Element::new(0.38066*255.0,0.47583*255.0,0.2855*255.0,0.0)); //equal
//     }

//     #[test]
//     fn materials_feature_110(){
//         let mut m = Material::new();
//         let pos = point(0.0,0.0,0.0);
//         let eyev = vector(0.0,0.0,-1.0);
//         let normalv = vector(0.0,0.0,-1.0);
//         let light = PointLight::new(point(0.0,0.0,-10.0), Color::new(1.0,1.0,1.0));

//         let result = lighting(m,&Shape::test().this_is(),&light,pos,eyev,normalv,true);
//         assert_eq!(Color::new(0.1,0.1,0.1),result);
//     }

//     #[test]
//     fn world_feature_111(){
//         let w = World::new();
//         let p = point(0.0,10.0,0.0);
//         assert_eq!(false, is_shadowed(&w,&p));

//         let p = point(10.0,-10.0,10.0);
//         assert_eq!(true, is_shadowed(&w,&p));

//         let p = point(-20.0,20.0,-20.0);
//         assert_eq!(false, is_shadowed(&w,&p));

//         let p = point(-2.0,2.0,-2.0);
//         assert_eq!(false, is_shadowed(&w,&p));


//     }
//     #[test]
//     fn world_feature_114(){
//         let mut w = World::new();
//         w.light_source = PointLight::new(point(0.0,0.0,-10.0),Color::new(1.0,1.0,1.0));
//         w.objects[1].set_transform(translation(0.0,0.0,10.0));
//         let ray = Ray::new(point(0.0,0.0,5.0),vector(0.0,0.0,1.0));
//         let i = Intersection::new(4.0,&w.objects[1]);
//         let comps = Computations::prepare_computations(&i,&ray, Intersections::empty());
//         assert_eq!(Color::new(0.1,0.1,0.1), shade_hit(&w,comps, REMAIN))
//     }

//     #[test]
//     fn intersections_feature_115(){
//         static EPSILON: f64 = 0.00001;
//         let ray = Ray::new(point(0.0,0.0,-5.0),vector(0.0,0.0,1.0));
//         let mut shape = Sphere::new();
//         shape.set_transform(&translation(0.0,0.0,1.0));
//         let b = shape.this_is();
//         let i = Intersection::new(5.0,&b);
//         let comps = Computations::prepare_computations(&i, &ray, Intersections::empty());
//         eprintln!("{:?}", i);
//         eprintln!("{:?}", comps);
//         assert_eq!(true, comps.over_point.z() < -EPSILON/2.0);
//         assert_eq!(true, comps.point.z() > comps.over_point.z());


//     }

//     #[test]
//     fn shapes_feature_119(){
//         let mut s = Shape::test();
//         assert_eq!(Material::new(), s.material);

//         let mut m = Material::new();
//         m.ambient = 1.0;
//         s.material = m.clone();
//         assert_eq!(m, s.material);

//         s.set_transform(translation(2.0,3.0,4.0));
//         assert_eq!(translation(2.0,3.0,4.0), s.transform);

//     }

//     #[test]
//     fn shapes_feature_120(){
//         let r = Ray::new(point(0.0,0.0,-5.0),vector(0.0,0.0,1.0));
//         let mut s = Shape::test();
        
//         s.set_transform(scale(2.0,2.0,2.0));
//         let s1 = s.this_is();
//         let xs = intersect_shape(&r,&s1); //passed
        
//         let mut s = Shape::test();
//         s.set_transform(translation(5.0,0.0,0.0));
//         let s1 = s.this_is();
//         let xs = intersect_shape(&r,&s1);

//     }

//     #[test]
//     fn shapes_feature_121(){
//         let mut s = Shape::test();
//         s.set_transform(translation(0.0,1.0,0.0));
//         let n = normal_at(&s, &point(0.0,1.70711,-0.70711));
//         assert!(vector(0.0,0.70711,-0.70711).matrix.equal(n.matrix));

//         s.set_transform(scale(1.0,0.5,1.0).dot(rotate_z(PI/5.0)).unwrap());
//         let n = normal_at(&s, &point(0.0,2.0_f64.sqrt()/2.0,-2.0_f64.sqrt()/2.0));
//         assert!(vector(0.0,0.97014,-0.24254).matrix.equal(n.matrix));
//     }

//     #[test]
//     fn planes_feature_122(){
//         let p = Plane::new();
//         let n1 = p.normal_at(&point(0.0,0.0,0.0));
//         assert_eq!(vector(0.0,1.0,0.0), n1);

//         let p = Plane::new();
//         let n2 = p.normal_at(&point(10.0,0.0,-10.0));
//         assert_eq!(vector(0.0,1.0,0.0), n2);

//         let p = Plane::new();
//         let n3 = p.normal_at(&point(-5.0,0.0,150.0));
//         assert_eq!(vector(0.0,1.0,0.0), n3);

//         let p = Plane::new();
//         let r = Ray::new(point(0.0,10.0,0.0), vector(0.0,0.0,1.0));
//         let xs = p.intersect(&r);
//         assert_eq!(0, xs.len());

//         let p = Plane::new();
//         let r = Ray::new(point(0.0,0.0,0.0), vector(0.0,0.0,1.0));
//         let xs = p.intersect(&r);
//         assert_eq!(0, xs.len());

//         let p = Plane::new();
//         let r = Ray::new(point(0.0,1.0,0.0), vector(0.0,-1.0,0.0));
//         let xs = p.clone().intersect(&r);
//         let p = Box::new(p).this_is();
//         let xs = p.make(xs); 
//         assert_eq!(1, xs.count);
//         assert_eq!(1.0, xs.h[0].t);
//         assert!(p.equal(xs.h[0].o));

//         let p = Plane::new();
//         let r = Ray::new(point(0.0,-1.0,0.0), vector(0.0,1.0,0.0));
//         let xs = p.clone().intersect(&r);
//         let p = Box::new(p).this_is();
//         let xs = p.make(xs); 
//         assert_eq!(1, xs.count);
//         assert_eq!(1.0, xs.h[0].t);
//         assert!(p.equal(xs.h[0].o));
        
//     }
//     #[test]
//     pub fn pattern_features_128(){
//         let pattern = StripePattern::new(Color::white(), Color::black());
//         assert_eq!(Color::white(), pattern.get_a());
//         assert_eq!(Color::black(), pattern.get_b());

//         assert_eq!(Color::white(),pattern.pattern_at(point(0.0,0.0,0.0)));
//         assert_eq!(Color::white(),pattern.pattern_at(point(0.0,1.0,0.0)));
//         assert_eq!(Color::white(),pattern.pattern_at(point(0.0,2.0,0.0)));

//         assert_eq!(Color::white(),pattern.pattern_at(point(0.0,0.0,0.0)));
//         assert_eq!(Color::white(),pattern.pattern_at(point(0.0,0.0,1.0)));
//         assert_eq!(Color::white(),pattern.pattern_at(point(0.0,0.0,2.0)));

//         assert_eq!(Color::white(),pattern.pattern_at(point(0.0,0.0,0.0)));
//         assert_eq!(Color::white(),pattern.pattern_at(point(0.9,0.0,1.0)));
//         assert_eq!(Color::black(),pattern.pattern_at(point(1.0,0.0,2.0)));
//         assert_eq!(Color::black(),pattern.pattern_at(point(-0.1,0.0,0.0)));
//         assert_eq!(Color::black(),pattern.pattern_at(point(-1.0,0.0,1.0)));
//         assert_eq!(Color::white(),pattern.pattern_at(point(-1.1,0.0,2.0)));
//     }

//     #[test]
//     pub fn materials_feature_129(){
//         let m = Material { color: Color::new(1.0,1.0,1.0),
//                                     ambient: 1.0 ,
//                                     diffuse: 0.9, 
//                                     specular: 0.0, 
//                                     shininess: 200.0,
//                                     pattern: Some(StripePattern::new(Color::white(), Color::black()).box_me()), 
//                                     reflective: 0.0,
//                                     transparency: 0.0,
//                                     refractive_index: 1.0};
//         let c1 = lighting(m.clone(), &Shape::test().this_is(),&PointLight::new(point(0.0,0.0,-10.0),
//         Color::white()), point(0.9,0.0,0.0),
//         vector(0.0,0.0,-1.0), vector(0.0,0.0,-1.0), false);

//         let c2 = lighting(m, &Shape::test().this_is(),&PointLight::new(point(0.0,0.0,-10.0),
//         Color::white()), point(1.1,0.0,0.0),
//         vector(0.0,0.0,-1.0), vector(0.0,0.0,-1.0), false);

//         assert_eq!(Color::black(),c2);
//         assert_eq!(Color::white(),c1);

//     }

//     #[test]
//     pub fn pattern_features_131(){
//         let mut o = Sphere::new();
//         o.set_transform(&scale(2.0,2.0,2.0));
//         let p = StripePattern::new(Color::white(), Color::black());
//         let c = p.pattern_at_shape(&o.this_is(), point(1.5,0.0,0.0));
//         assert_eq!(Color::white(),c);

//         let mut o = Sphere::new();
        
//         let mut p = StripePattern::new(Color::white(), Color::black());
//         p.set_transform(scale(2.0,2.0,2.0));
//         let c = p.pattern_at_shape(&o.this_is(), point(1.5,0.0,0.0));
//         assert_eq!(Color::white(),c);
        
//         let mut o = Sphere::new();
//         o.set_transform(&scale(2.0,2.0,2.0));
//         let mut p = StripePattern::new(Color::white(), Color::black());
//         p.set_transform(translation(0.5,2.0,2.0));
//         let c = p.pattern_at_shape(&o.this_is(), point(2.5,0.0,0.0));
//         assert_eq!(Color::white(),c);
//     }

//     #[test]
//     pub fn pattern_features_133(){
//         let mut p = StripePattern::base();
//         assert_eq!(Matrix::zero(4,4).identity(), p.get_transform());

//         p.set_transform(translation(1.0,2.0,3.0));
//         assert_eq!(translation(1.0,2.0,3.0), p.get_transform());


//         let mut o = Sphere::new();
//         o.set_transform(&scale(2.0,2.0,2.0));
//         let p = TestPattern::new();
//         let c = p.pattern_at_shape(&o.this_is(), point(2.0,3.0,4.0));
//         assert_eq!(Color::new(1.0,1.5,2.0),c); //clamped in real

//         let mut o = Sphere::new();
//         let mut p = TestPattern::new();
//         p.set_transform(scale(2.0,2.0,2.0));
//         let c = p.pattern_at_shape(&o.this_is(), point(2.0,3.0,4.0));
//         assert_eq!(Color::new(1.0,1.5,2.0),c);//clamped in real

//         let mut o = Sphere::new();
//         o.set_transform(&scale(2.0,2.0,2.0));
//         let mut p = TestPattern::new();
//         p.set_transform(translation(0.5,1.0,1.5));
//         let c = p.pattern_at_shape(&o.this_is(), point(2.5,3.0,3.5));
//         assert_eq!(Color::new(0.75,0.5,0.25),c);

//     }

//     #[test]
//     pub fn pattern_features_135(){
//         let p = GradientPattern::base();
//         let c = p.pattern_at(point(0.0,0.0,0.0));
//         assert_eq!(Color::white(),c);

//         let p = GradientPattern::base();
//         let c = p.pattern_at(point(0.25,0.0,0.0));
//         assert_eq!(Color::new(0.75,0.75,0.75),c);

//         let p = GradientPattern::base();
//         let c = p.pattern_at(point(0.5,0.0,0.0));
//         assert_eq!(Color::new(0.5,0.5,0.5),c);

//         let p = GradientPattern::base();
//         let c = p.pattern_at(point(0.75,0.0,0.0));
//         assert_eq!(Color::new(0.25,0.25,0.25),c);
//     }

//     #[test]
//     pub fn pattern_features_136(){
//         let p = RingPattern::base();
//         let c = p.pattern_at(point(0.0,0.0,0.0));
//         assert_eq!(Color::white(),c);

//         let c = p.pattern_at(point(1.0,0.0,0.0));
//         assert_eq!(Color::black(),c);


//         let c = p.pattern_at(point(0.0,0.0,1.0));
//         assert_eq!(Color::black(),c);

//         let c = p.pattern_at(point(0.708,0.0,0.708));
//         assert_eq!(Color::black(),c);
//     }

//     #[test]
//     pub fn pattern_features_137(){
//         let p = CheckersPattern::base();
//         let c = p.pattern_at(point(0.0,0.0,0.0));
//         assert_eq!(Color::white(),c);
//         let c = p.pattern_at(point(0.99,0.0,0.0));
//         assert_eq!(Color::white(),c);
//         let c = p.pattern_at(point(1.01,0.0,0.0));
//         assert_eq!(Color::black(),c);

//         let c = p.pattern_at(point(0.0,0.0,0.0));
//         assert_eq!(Color::white(),c);
//         let c = p.pattern_at(point(0.0,0.99,0.0));
//         assert_eq!(Color::white(),c);
//         let c = p.pattern_at(point(0.0,1.01,0.0));
//         assert_eq!(Color::black(),c);

        
//         let c = p.pattern_at(point(0.0,0.0,0.0));
//         assert_eq!(Color::white(),c);
//         let c = p.pattern_at(point(0.0,0.0,0.99));
//         assert_eq!(Color::white(),c);
//         let c = p.pattern_at(point(0.0,0.0,1.01));
//         assert_eq!(Color::black(),c);
//     }

//     #[test]
//     pub fn materials_feature_143(){
//         let material = Material::new();
//         assert_eq!(0.0, material.reflective)
//     }

    
//     #[test]
//     pub fn intersections_feature_143(){
//         let s = Plane::new().this_is();
//         let r = Ray::new(point(0.0,1.0,-1.0), vector(0.0,-2.0_f64.sqrt()/2.0,2.0_f64.sqrt()/2.0));
//         let i = Intersection::new(2.0_f64.sqrt(),&s);
//         let comps = Computations::prepare_computations(&i,&r, Intersections::empty());
//         assert_eq!(vector(0.0,2.0_f64.sqrt()/2.0,2.0_f64.sqrt()/2.0), comps.get_reflectv())
//     }

//     #[test]
//     pub fn worlds_feature_144(){
//         let mut w = World::new();
//         let r = Ray::new(point(0.0,0.0,0.0), vector(0.0,0.0,1.0));
//         let mut m1 = w.objects[1].get_material();
//         m1.ambient = 1.0;
//         w.objects[1].set_material(m1);
//         let s =  &w.objects[1];
//         let i = Intersection::new(1.0, s);
//         let comps = Computations::prepare_computations(&i,&r, Intersections::empty());
//         let color = reflected_color(&w,comps,REMAIN);
//         assert_eq!(color, Color::black());

//         let mut w = World::new();
//         let r = Ray::new(point(0.0,0.0,-3.0), vector(0.0,-2.0_f64.sqrt()/2.0,2.0_f64.sqrt()/2.0));
//         let mut p = Plane::new();
//         p.material.reflective = 0.5;
//         p.transform = translation(0.0,-1.0,0.0);
//         w.objects.push(p.this_is());
//         let i = Intersection::new(2.0_f64.sqrt(), &w.objects[2]);
//         let comps = Computations::prepare_computations(&i,&r, Intersections::empty());
//         let color = reflected_color(&w,comps,REMAIN);
//         //assert_eq!(color, Color::new(0.19032,0.2379,0.14274)); // f64 rounding
       
//         let mut w = World::new();
//         let r = Ray::new(point(0.0,0.0,-3.0), vector(0.0,-2.0_f64.sqrt()/2.0,2.0_f64.sqrt()/2.0));
//         let mut p = Plane::new();
//         p.material.reflective = 0.5;
//         p.transform = translation(0.0,-1.0,0.0);
//         w.objects.push(p.this_is());
//         let i = Intersection::new(2.0_f64.sqrt(), &w.objects[2]);
//         let comps = Computations::prepare_computations(&i,&r, Intersections::empty());
//         let color = shade_hit(&w,comps,REMAIN);
//         //assert_eq!(color, Color::new(0.87677,0.92436,0.82918)); // f64 rounding , within 3 decimal places

//         let mut w = World::new();
//         w.light_source = PointLight::new(point(0.0,0.0,0.0), Color::white());
//         let r = Ray::new(point(0.0,0.0,0.0), vector(0.0,1.0,0.0));
//         let mut lower = Plane::new();
//         lower.material.reflective = 1.0;
//         lower.transform = translation(0.0,-1.0,0.0);
//         let mut upper = Plane::new();
//         upper.material.reflective = 1.0;
//         upper.transform = translation(0.0,1.0,0.0);
//         w.objects.push(lower.this_is());
//         w.objects.push(upper.this_is());
//         let color = color_at(&w,&r,REMAIN);

//         let mut w = World::new();
//         let r = Ray::new(point(0.0,0.0,-3.0), vector(0.0,-2.0_f64.sqrt()/2.0,2.0_f64.sqrt()/2.0));
//         let mut p = Plane::new();
//         p.material.reflective = 0.5;
//         p.transform = translation(0.0,-1.0,0.0);
//         w.objects.push(p.this_is());
//         let i = Intersection::new(2.0_f64.sqrt(), &w.objects[2]);
//         let comps = Computations::prepare_computations(&i,&r, Intersections::empty());
//         let color = reflected_color(&w,comps,0);
//         assert_eq!(Color::black(), color);

//     }

//     #[test]
//     pub fn materials_feature_150(){
//         let m = Material::new();
//         assert_eq!(m.transparency, 0.0);
//         assert_eq!(m.refractive_index, 1.0);


//         let s = Sphere::glass();
//         assert_eq!(s.material.transparency,1.0);
//         assert_eq!(s.material.refractive_index,1.5);
    
//     }

//     #[test]
//     pub fn intersections_feature_152(){
//         let mut a = Sphere::glass();
//         a.set_transform(&scale(2.0,2.0,2.0));
//         a.material.refractive_index = 1.5;
//         let a = a.this_is();

//         let mut b = Sphere::glass();
//         b.set_transform(&translation(0.0,0.0,-0.25));
//         b.material.refractive_index = 2.0;
//         let b = b.this_is();

//         let mut c = Sphere::glass();
//         c.set_transform(&translation(0.0,0.0,0.25));
//         c.material.refractive_index = 2.5;
//         let c = c.this_is();

//         let r = Ray::new(point(0.0,0.0,-4.0,),vector(0.0,0.0,1.0));
//         let xs = Intersections{count: 6,
//                                             h: vec![Intersection{t:2.0, o: &a},
//                                                     Intersection{t:2.75, o: &b},
//                                                     Intersection{t:3.25, o: &c},
//                                                     Intersection{t:4.75, o: &b},
//                                                     Intersection{t:5.25, o: &c},
//                                                     Intersection{t:6.0, o: &a}]};
        
//         let answers = [
//                         [1.0,1.5],
//                         [1.5,2.0],
//                         [2.0,2.5],
//                         [2.5,2.5],
//                         [2.5,1.5],
//                         [1.5,1.0]];
//         for (i,e) in answers.iter().enumerate(){
//             let xs_c = xs.clone();
//             let comps = Computations::prepare_computations(&xs.h[i], &r, xs_c);
//             assert_eq!(comps.n1,e[0]);
//             assert_eq!(comps.n2,e[1]);
//         }
        
//     }

//     #[test]
//     pub fn intersections_feature_154(){
//         let r = Ray::new(point(0.0,0.0,-5.0,),vector(0.0,0.0,1.0));
//         let mut s = Sphere::glass();
//         s.set_transform(&translation(0.0,0.0,1.0));
//         let s_b = s.this_is();
//         let i = Intersection{t:5.0, o: &s_b};
//         let xs = Intersections { count: 1, h: vec![i.clone()]};
//         let comps = Computations::prepare_computations(&i, &r, xs);
//         assert!(comps.under_point.z() > EPSILON/2.0);
//         assert!(comps.point.z() < comps.under_point.z());
//     }

//     #[test]
//     pub fn worlds_feature_155(){
//         let w = World::new();
//         let s_b = &w.objects[0]; // cannot move out of index bc no copy
  
//         let r = Ray::new(point(0.0,0.0,-5.0,),vector(0.0,0.0,1.0));
//         let xs = Intersections { count: 2, h: vec![Intersection{t:4.0, o: &s_b},
//                                                                 Intersection{t:6.0, o: &s_b}]};
//         let xs_c = xs.clone();
//         let comps = Computations::prepare_computations(&xs.h[0], &r, xs_c);
//         assert_eq!(refracted_color(&w,comps,5),Color::black());
//     }

    
//     #[test]
//     pub fn worlds_feature_156(){
//         let mut w = World::new();
//         let mut m_1 = w.objects[0].get_material();
//         m_1.transparency = 1.0;
//         m_1.refractive_index = 1.5;
//         w.objects[0].set_material(m_1);

//         let s_b = &w.objects[0]; // cannot move out of index bc no copy
  
//         let r = Ray::new(point(0.0,0.0,-5.0,),vector(0.0,0.0,1.0));
//         let xs = Intersections { count: 2, h: vec![Intersection{t:4.0, o: &s_b},
//                                                                 Intersection{t:6.0, o: &s_b}]};
//         let xs_c = xs.clone();
//         let comps = Computations::prepare_computations(&xs.h[0], &r, xs_c);
//         assert_eq!(refracted_color(&w,comps,0),Color::black());
//     }

//     #[test]
//     pub fn worlds_feature_157(){
//         let mut w = World::new();
//         let mut m_1 = w.objects[0].get_material();
//         m_1.transparency = 1.0;
//         m_1.refractive_index = 1.5;
//         w.objects[0].set_material(m_1);
        
//         let s_b = &w.objects[0]; 
//         let r = Ray::new(point(0.0,0.0,2.0_f64.sqrt()/2.0,),vector(0.0,1.0,0.0));
//         let xs = Intersections { count: 2, h: vec![Intersection{t:-2.0_f64.sqrt()/2.0, o: &s_b},
//                                                                 Intersection{t:2.0_f64.sqrt()/2.0, o: &s_b}]};
//         let xs_c = xs.clone();
//         let comps = Computations::prepare_computations(&xs.h[1], &r, xs_c);
//         assert_eq!(refracted_color(&w,comps,5),Color::black());
//     }

//     #[test]
//     pub fn worlds_feature_158(){
//         let mut w = World::new();


//         let mut m_1 = w.objects[0].get_material();
//         m_1.ambient = 1.0;
//         m_1.pattern = Some(TestPattern::new().box_me());
//         w.objects[0].set_material(m_1);

//         let mut m_2 = w.objects[1].get_material();
//         m_2.transparency = 1.0;
//         m_2.refractive_index = 1.5;
//         w.objects[1].set_material(m_2);

//         let s1 = & w.objects[0]; 
//         let s2 = & w.objects[1];

//         let r = Ray::new(point(0.0,0.0,0.1,),vector(0.0,1.0,0.0));
//         let xs = Intersections { count: 2, h: vec![Intersection{t:-0.9899, o: &s1},
//                                                                 Intersection{t:-0.4899, o: &s2},
//                                                                 Intersection{t:0.4899, o: &s2},
//                                                                 Intersection{t:0.9899, o: &s1}]};
//         let xs_c = xs.clone();
//         let comps = Computations::prepare_computations(&xs.h[2], &r, xs_c);

//         //assert!(refracted_color(&w,comps,5).equal(Color::new(0.0,0.99888,0.04725))); // 0.0000 is good ***CHECK
//     }
    
//     #[test]
//     pub fn worlds_feature_159(){
//         let mut w = World::new();
//         let mut f = Plane::new();
//         f.transform = translation(0.0,-1.0,0.0);
//         f.material.transparency = 0.5;
//         f.material.refractive_index = 1.5;
//         let f_b_c = f.clone().this_is();
//         w.objects.push(f.this_is());
        

//         let mut s = Sphere::new();
//         s.transform = translation(0.0,-3.5,-0.5);
//         s.material.ambient = 0.5;
//         s.material.color = Color::new(1.0,0.0,0.0);
//         w.objects.push(s.this_is());

//         let r = Ray::new(point(0.0,0.0,-3.0,),vector(0.0,-2.0_f64.sqrt()/2.0,2.0_f64.sqrt()/2.0));
//         let xs = Intersections { count: 1, h: vec![Intersection{t:2.0_f64.sqrt(), o: &f_b_c},
//                                                                 ]};
//         let xs_c = xs.clone();
//         let comps = Computations::prepare_computations(&xs.h[0], &r, xs_c);
//         assert!(shade_hit(&w,comps,5).equal(Color::new(0.93642,0.68642,0.68642)));
//     }

//     #[test]
//     pub fn intersections_feature_161(){
//         let s = Sphere::glass().this_is();
//         let r = Ray::new(point(0.0,0.0,2.0_f64.sqrt()/2.0,),vector(0.0,1.0,0.0));
//         let xs = Intersections { count: 2, h: vec![Intersection{t:-2.0_f64.sqrt()/2.0, o: &s}, Intersection{t:2.0_f64.sqrt()/2.0, o: &s}]};
//         let xs_c = xs.clone();
//         let comps = Computations::prepare_computations(&xs.h[1], &r, xs_c); //maybe no need for reference 
//         let reflectance = schlick(comps);
//         assert_eq!(1.0,reflectance);

//         let r = Ray::new(point(0.0,0.0,0.0,),vector(0.0,1.0,0.0));
//         let xs = Intersections { count: 2, h: vec![Intersection{t:-1.0, o: &s}, Intersection{t:1.0, o: &s}]};
//         let xs_c = xs.clone();
//         let comps = Computations::prepare_computations(&xs.h[1], &r, xs_c); //maybe no need for reference 
//         let reflectance = schlick(comps);
//         assert_eq!(0.04,reflectance as f32);

//         let r = Ray::new(point(0.0,0.99,-2.0,),vector(0.0,0.0,1.0));
//         let xs = Intersections { count: 1, h: vec![Intersection{t:1.8589, o: &s}]};
//         let xs_c = xs.clone();
//         let comps = Computations::prepare_computations(&xs.h[0], &r, xs_c); //maybe no need for reference 
//         let reflectance = schlick(comps);
//         assert!(equal_floats(&0.48873,&reflectance));
//     }

//     #[test]
//     pub fn worlds_feature_164(){
//         let mut w = World::new();
//         let mut f = Plane::new();
//         f.transform = translation(0.0,-1.0,0.0);
//         f.material.transparency = 0.5;
//         f.material.refractive_index = 1.5;
//         f.material.reflective = 0.5;
//         let f_b_c = f.clone().this_is();
//         w.objects.push(f.this_is());
        

//         let mut s = Sphere::new();
//         s.transform = translation(0.0,-3.5,-0.5);
//         s.material.ambient = 0.5;
//         s.material.color = Color::new(1.0,0.0,0.0);
//         w.objects.push(s.this_is());

//         let r = Ray::new(point(0.0,0.0,-3.0,),vector(0.0,-2.0_f64.sqrt()/2.0,2.0_f64.sqrt()/2.0));
//         let xs = Intersections { count: 1, h: vec![Intersection{t:2.0_f64.sqrt(), o: &f_b_c},
//                                                                 ]};
//         let xs_c = xs.clone();
//         let comps = Computations::prepare_computations(&xs.h[0], &r, xs_c);
//         //assert!(refracted_color(&w,comps,5).equal(Color::new(0.93394,0.69643,0.69243))); // up to 0.0000 ***CHECK
//     }

//     #[test]
//     pub fn cubes_feature_168(){
//         let c = Cube::new();
        
//         let answers = [
//                         (point(5.0,0.5,0.0), vector(-1.0,0.0,0.0), 4.0, 6.0),
//                         (point(-5.0,0.5,0.0), vector(1.0,0.0,0.0), 4.0, 6.0),
//                         (point(0.5,5.0,0.0), vector(0.0,-1.0,0.0), 4.0, 6.0),
//                         (point(0.5,-5.0,0.0), vector(0.0,1.0,0.0), 4.0, 6.0),
//                         (point(0.5,0.0,5.0), vector(0.0,0.0,-1.0), 4.0, 6.0),
//                         (point(0.5,0.0,-5.0), vector(0.0,0.0,1.0), 4.0, 6.0),
//                         (point(0.0,0.5,0.0), vector(0.0,0.0,1.0), -1.0, 1.0),
//                      ];

//         for (i,e) in answers.iter().enumerate(){
//             let r = Ray::new(e.0.clone(),e.1.clone());
//             let xs = c.intersect(&r);
            
//             assert_eq!(xs[0],e.2);
//             assert_eq!(xs[1],e.3);

//         }
//     }

//     #[test]
//     pub fn cubes_feature_172(){
//         let c = Cube::new();
        
//         let answers = [
//                         (point(-2.0,0.0,0.0), vector(0.2673,0.5345,0.8018), vec![]),
//                         (point(0.0,-2.0,0.0), vector(0.8018,0.2673,0.5345), vec![]),
//                         (point(0.0,0.0,-2.0), vector(0.5345,0.8018,0.2673), vec![]),
//                         (point(2.0,0.0,2.0), vector(0.0,0.0,-1.0), vec![]),
//                         (point(0.0,2.0,2.0), vector(0.0,-1.0,0.0), vec![]),
//                         (point(2.0,2.0,0.0), vector(-1.0,0.0,0.0), vec![]),
                        
//                      ];

//         for (i,e) in answers.iter().enumerate(){
//             let r = Ray::new(e.0.clone(),e.1.clone());
//             let xs = c.intersect(&r);  
//             assert_eq!(xs,e.2);
//         }
//     }

//     #[test]
//     pub fn cubes_feature_173(){
//         let c = Cube::new();
//         let answers = [
//             (point(1.0,0.5,-0.8),vector(1.0,0.0,0.0)),
//             (point(-1.0,-0.2,0.9),vector(-1.0,0.0,0.0)),
//             (point(-0.4,1.0,-0.1),vector(0.0,1.0,0.0)),
//             (point(0.3,-1.0,-0.7),vector(0.0,-1.0,0.0)),
//             (point(-0.6,0.3,1.0),vector(0.0,0.0,1.0)),
//             (point(0.4,0.4,-1.0),vector(0.0,0.0,-1.0)),
//             (point(1.0,1.0,1.0),vector(1.0,0.0,0.0)),
//             (point(-1.0,-1.0,-1.0),vector(-1.0,0.0,0.0)),
//         ];
        
//         for (i,e) in answers.iter().enumerate(){
            
//             let n = c.normal_at(&e.0);
//             assert_eq!(n,e.1)
//         }
//     }

//     #[test]
//     pub fn cylinders_features_178(){
//         let c = Cylinder::new();
//         let answers = [
//             (point(1.0,0.0,0.0),vector(0.0,1.0,0.0)),
//             (point(0.0,0.0,0.0),vector(0.0,1.0,0.0)),
//             (point(0.0,0.0,-5.0),vector(1.0,1.0,1.0))];

//         for (i,e) in answers.iter().enumerate(){
//             let dir = e.1.normal();
//             let o = e.0.clone();
//             let r = Ray::new(o,dir);
//             let xs = c.intersect(&r);
//             assert_eq!(xs,vec![])
//         }
//     }

//     #[test]
//     pub fn cylinders_features_180(){
//         let c = Cylinder::new();
//         let answers = [
//             (point(1.0,0.0,-5.0),vector(0.0,0.0,1.0),5.0,5.0),
//             (point(0.0,0.0,-5.0),vector(0.0,0.0,1.0),4.0,6.0),
//             (point(0.5,0.0,-5.0),vector(0.1,1.0,1.0),6.80798,7.08872)];

//         for (i,e) in answers.iter().enumerate(){
//             let dir = e.1.normal();
//             let o = e.0.clone();
//             let r = Ray::new(o,dir);
//             let xs = c.intersect(&r);
//             assert_eq!(xs.len(),2);
//             assert!(equal_floats(&xs[0],&e.2));
//             assert!(equal_floats(&xs[1],&e.3));
//         }
//     }

//     #[test]
//     pub fn cylinders_features_181(){
//         let c = Cylinder::new();
//         let answers = [
//             (point(1.0,0.0,0.0),vector(1.0,0.0,0.0)),
//             (point(0.0,5.0,-1.0),vector(0.0,0.0,-1.0)),
//             (point(0.0,-2.0,1.0),vector(0.0,0.0,1.0)),
//             (point(-1.0,1.0,0.0),vector(-1.0,0.0,0.0))];

//         for (i,e) in answers.iter().enumerate(){
//             let n = c.normal_at(&e.0);
//             assert_eq!(n,e.1)
      
//         }
//     }


//     #[test]
//     pub fn cylinders_features_182(){
//         let c = Cylinder::new();
//         assert_eq!(c.min,f64::NEG_INFINITY);
//         assert_eq!(c.max,f64::INFINITY);

//         let mut c = Cylinder::new();
//         c.min = 1.0;
//         c.max = 2.0;

//         let answers = [
//             (point(0.0,1.0,0.0),vector(0.1,1.0,0.0),0),
//             (point(0.0,3.0,-5.0),vector(0.0,0.0,1.0),0),
//             (point(0.0,0.0,-5.0),vector(0.0,0.0,1.0),0),
//             (point(0.0,2.0,-5.0),vector(0.0,0.0,1.0),0),
//             (point(0.0,1.0,-5.0),vector(0.0,0.0,1.0),0),
//             (point(0.0,1.5,-2.0),vector(0.0,0.0,1.0),2)];

//         for (i,e) in answers.iter().enumerate(){
//             let dir =  e.1.normal();
//             let r = Ray::new(e.0.clone(),dir);
//             let xs = c.intersect(&r);
           
//             assert_eq!(e.2,xs.len())
      
//         }
//     }

//     #[test]
//     pub fn cylinders_features_185(){
//         let mut c = Cylinder::new();
//         assert_eq!(false,c.closed);
//         c.min = 1.0;
//         c.max = 2.0;
//         c.closed = true;
        

//         let answers = [
//             (point(0.0,3.0,0.0),vector(0.0,-1.0,0.0),2),
//             (point(0.0,3.0,-2.0),vector(0.0,-1.0,2.0),2),
//             (point(0.0,4.0,-2.0),vector(0.0,-1.0,1.0),2),
//             (point(0.0,0.0,-2.0),vector(0.0,1.0,2.0),2),
//             (point(0.0,-1.0,-2.0),vector(0.0,1.0,1.0),2)];

//         for (i,e) in answers.iter().enumerate(){
//             let dir =  e.1.normal();
//             let r = Ray::new(e.0.clone(),dir);
//             let xs = c.intersect(&r);
//             assert_eq!(e.2,xs.len());
      
//         }

//     }

//     #[test]
//     pub fn cylinders_features_187(){
//         let mut c = Cylinder::new();
//         assert_eq!(false,c.closed);
//         c.min = 1.0;
//         c.max = 2.0;
//         c.closed = true;
        

//         let answers = [
//             (point(0.0,1.0,0.0),vector(0.0,-1.0,0.0)),
//             (point(0.5,1.0,0.0),vector(0.0,-1.0,0.0)),
//             (point(0.0,1.0,0.5),vector(0.0,-1.0,0.0)),
//             (point(0.0,2.0,0.0),vector(0.0,1.0,0.0)),
//             (point(0.5,2.0,0.0),vector(0.0,1.0,0.0)),
//             (point(0.0,2.0,0.5),vector(0.0,1.0,0.0))];

//         for (i,e) in answers.iter().enumerate(){
//             let n =  c.normal_at(&e.0);
            
//             assert_eq!(e.1,n);
      
//         }

//     }

//     #[test]
//     pub fn cones_features_189(){
//         let c = Cone::new();
 
        

//         let answers = [
//             (point(0.0,0.0,-5.0),vector(0.0,0.0,1.0),5.0,5.0),
//             (point(0.0,0.0,-5.0),vector(1.0,1.0,1.0),8.66025,8.66025),
//             (point(1.0,1.0,-5.0),vector(-0.5,-1.0,1.0),4.55006,49.44994)];

//         for (i,e) in answers.iter().enumerate(){
//             let dir =  e.1.normal();
//             let r = Ray::new(e.0.clone(),dir);
//             let xs = c.intersect(&r);
//             assert!(equal_floats(&e.2,&xs[0]));
//             assert!(equal_floats(&e.3,&xs[1]));
//         }

//         let c = Cone::new();
//         let dir = vector(0.0,1.0,1.0).normal();
//         let r = Ray::new(point(0.0,0.0,-1.0),dir);
//         let xs = c.intersect(&r);
//         assert_eq!(equal_floats(&xs[0],&0.35355),true);

//         let mut c = Cone::new();
//         c.min = -0.5;
//         c.max = 0.5;
//         c.closed = true;
//         let answers = [
//             (point(0.0,0.0,-5.0),vector(0.0,1.0,0.0),0),
//             (point(0.0,0.0,-0.25),vector(0.0,1.0,1.0),2),
//             (point(0.0,0.0,-0.25),vector(0.0,1.0,0.0),4)];

//         for (i,e) in answers.iter().enumerate(){
//             let dir =  e.1.normal();
//             let r = Ray::new(e.0.clone(),dir);
//             let xs = c.intersect(&r);
//             assert_eq!(xs.len(),e.2);
            
//         }


        
//         let c = Cone::new();

//         let answers = [
//             (point(0.0,0.0,0.0),vector(0.0,0.0,0.0)),
//             (point(1.0,1.0,1.0),vector(1.0,-(2.0_f64.sqrt()),1.0)),
//             (point(-1.0,-1.0,0.0),vector(-1.0,1.0,0.0))];

//         for (i,e) in answers.iter().enumerate(){
//             let n =  c.normal_at(&e.0);
            
//             assert_eq!(e.1,n);
      
//         }
//     }

    #[test]
    pub fn groups_feature_195(){
        let mut g = Group::new();
        assert_eq!(Shape::test().transform,g.transform);
        assert_eq!(0, g.members.borrow().len());
      
        assert_eq!(true, Shape::test().parent.is_none());
        let mut g_w = wrap_this(g);
        let s_w = wrap_this(Shape::test());
        add_child(&mut g_w, &s_w); // *** ALLOWS FOR CHANGE IN MEMBERS FIELD EVEN IF GROUP IS IMMUTABLE?? 
        assert_ne!(0, g_w.borrow_mut().get_members().borrow().len());
 
        //&mut g and & g (with g being mut) are different
        assert_eq!(*g_w.borrow_mut().get_members(),RefCell::new(vec![RefCell::new(Shape::test().this_is())]));
        let get_parent_of_s = s_w.borrow_mut().get_parent().upgrade().unwrap(); //clone bc this data lives on heap,
        
        assert_eq!(get_parent_of_s, g_w)
   
        // design decision: using option for parent: only one parent allowed, bc having vec! for parents is difficult when accessing the inner values, (have to move out of index
        // but the data structure does not implement copy trait)

    }


    #[test]
    pub fn groups_feature_196(){
        let g = Group::new();
        let r = Ray::new(   point(0.0,0.0,0.0),
                                vector(0.0,0.0,0.0));
        let xs = g.intersect(&r);
        assert_eq!(0,xs.len());

        let mut g_rc = wrap_this(g);

        let s1 = Sphere::new();
        let s1_rc = wrap_this(s1);
        let mut s2 = Sphere::new();
        s2.set_transform(translation(0.0,0.0,-3.0));
        let s2_rc = wrap_this(s2);
       

        let mut s3 = Sphere::new();
        s3.set_transform(translation(5.0,0.0,0.0));
        let s3_rc = wrap_this(s3);
        
        
        add_child(&mut g_rc, &s1_rc);
        add_child(&mut g_rc,&s2_rc);
        add_child(&mut g_rc,&s3_rc);
       
        let r = Ray::new(   point(0.0,0.0,-5.0),
                                vector(0.0,0.0,1.0));
        let list = take_members_out(&g_rc);
        
        let results = intersect(list,&r,vec![]);
    
        assert_eq!(4,results.count);
        // assert!(results.h[0].o == s2_rc);
        // assert_eq!(Rc::new(*s2_rc.borrow_mut()),results.h[1].o);
        // assert_eq!(Rc::new(*s1_rc.borrow_mut()),results.h[2].o);
        // assert_eq!(Rc::new(*s1_rc.borrow_mut()),results.h[3].o);
        // from prior version: 
        // issues in the beginning with intersections due to calling the local transform and not intersect_shape (where ray is transformed)
        // note: update_parent_.. adds a Rc to a CLONED parent, therefore the parent(in the field) does not have the member's parent updated
        // so in members -> parent [field] = parent[with members with parents:none], but the overall parent does have the individual members within the member field
        // ran into the mutable borrow constraint with refcell, checked at compile time? or is it run time -> the later one
        // issues with lifetimes not being consistent when using references -> tried references bc did not want to take ownership 
        // vec![refcell] had to create a helper function to convert to just vec![] -> question now is why am i using refcell -> edit: hmm probably could use just vec!
    }


    #[test]
    pub fn groups_feature_197(){
        let mut g = Group::new();
    
        g.set_transform(scale(2.0,2.0,2.0));
        let mut g_rc = Rc::new(RefCell::new(g.this_is()));
        let mut s1 = Sphere::new();
        s1.set_transform(translation(5.0,0.0,0.0));
        let s1_rc = wrap_this(s1);
       
        add_child(&mut g_rc, &s1_rc);
        
     

        //let g = Rc::try_unwrap(g_rc).unwrap(); //edit: no bc g_rc points to a clone of g; the members of g are updated to have parent of a CLONE of g
        
        let r = Ray::new(   point(10.0,0.0,-10.0),
                                vector(0.0,0.0,1.0));
        let list = take_members_out(&g_rc);

        let results = intersect(list,&r,vec![]);
     
    
        assert_eq!(2,results.count);
        //adding helper function to apply group's tranform to ray before the object's transform -> edit: implemented the transform in the TAKING OUT helper function

    }

    #[test]
    pub fn shapes_feature_198(){
        // older: 
        // let mut g1 = Group::new();
        // g1.set_transform(rotate_y(PI/2.0));
        // let mut g2 = Group::new().set_transform_group(scale(2.0,2.0,2.0));
        // let mut s = Sphere::new();
        // s.set_transform(translation(5.0,0.0,0.0));
  

        // ///adding g2 to g1
        // let g1_rc = Rc::new(RefCell::new(g1.this_is()));

        // let mut g2_rc = Rc::new(RefCell::new(g2.this_is()));                            /// g2_rc
        // eprintln!("{:?}",Rc::strong_count(&g1_rc));

        // let s = wrap_this(s);
        // add_child(&mut g2_rc,&s); //g2 <=> s
        
        // g2_rc.borrow_mut().set_parent(&g1_rc);
        // eprintln!("g1 parent {:?}",g1_rc);
        // g1_rc.borrow_mut().set_sub_group(&g2_rc); //g1 <=> g2
        // eprintln!("g1 AFTER SUBGROUP {:?}",g1_rc);
        // eprintln!("g2 parent {:?}",g2_rc);

        // let p = world_to_object(&s,&point(-2.0,0.0,-10.0));
        // assert!(p.matrix.equal(point(0.0,0.0,-1.0).matrix));

        let mut g1 = Group::new();
        g1.set_transform(rotate_y(PI/2.0));
        let mut g2 = Group::new();
        g2.set_transform(scale(2.0,2.0,2.0));
        let mut s = Sphere::new();
        s.set_transform(translation(5.0,0.0,0.0));
  

        ///adding g2 to g1
        let mut g1_rc = wrap_this(g1);
        let mut g2_rc = wrap_this(g2);                        
        let s = wrap_this(s);
        
        // g2 <=> s
        add_child(&mut g2_rc,&s); 
        
        //g1 <=> g2
        add_child(&mut g1_rc,&g2_rc); 
        let p = world_to_object(&s,&point(-2.0,0.0,-10.0));
        assert!(p.matrix.equal(point(0.0,0.0,-1.0).matrix));


    }

    #[test]
    pub fn shapes_feature_199(){
        let mut g1 = Group::new();
        g1.set_transform(rotate_y(PI/2.0));
        let mut g2 = Group::new();
        g2.set_transform(scale(1.0,2.0,3.0));
        let mut s = Sphere::new();
        s.set_transform(translation(5.0,0.0,0.0));
  

        ///adding g2 to g1
        let mut g1_rc = wrap_this(g1);
        let mut g2_rc = wrap_this(g2);                        
        let s = wrap_this(s);
        
        // g2 <=> s
        add_child(&mut g2_rc,&s); 
        //g1 <=> g2
        add_child(&mut g1_rc,&g2_rc); 

        let n = normal_to_world(&s,&vector(3.0_f64.sqrt()/3.0,3.0_f64.sqrt()/3.0,3.0_f64.sqrt()/3.0));
        //assert!(n.matrix.equal(vector(0.2857,0.4286,-0.8571).matrix)); // result:  matrix: [0.28571428571428575], [0.42857142857142855], [-0.8571428571428571], [0.0]

        let n_at = normal_at_w(&s, &point(1.7321,1.1547,-5.5774));
        //assert!(n_at.matrix.equal(vector(0.2857,0.4286,-0.8571).matrix)); //result:  matrix: [0.28570368184140726], [0.428543151781141], [-0.8571605294481016], [0.0]


    }

    #[test]
    pub fn shapes_feature_199_2(){
        let mut g1 = Group::new();
        g1.set_transform(rotate_y(PI/2.0));
        let mut g2 = Group::new();
        g2.set_transform(scale(1.0,2.0,3.0));
        let mut s = Sphere::new();
        s.set_transform(translation(5.0,0.0,0.0));
  

        ///adding g2 to g1
        let mut g1_rc = wrap_this(g1);
        let mut g2_rc = wrap_this(g2);                        
        let s = wrap_this(s);
        
        // g2 <=> s
        add_child(&mut g2_rc,&s); 
        //g1 <=> g2
        add_child(&mut g1_rc,&g2_rc); 

        let n = normal_to_world(&s,&vector(3.0_f64.sqrt()/3.0,3.0_f64.sqrt()/3.0,3.0_f64.sqrt()/3.0));
        //assert!(n.matrix.equal(vector(0.2857,0.4286,-0.8571).matrix)); // result:  matrix: [0.28571428571428575], [0.42857142857142855], [-0.8571428571428571], [0.0]

    }

    #[test]
    pub fn bounding_box_feature_201(){
        let bbox = BoundingBox::new(point(-1.0, -1.0, -1.0) ,point(1.0, 1.0, 1.0));
        let matrix = rotate_x(PI/4.0).dot(rotate_y(PI/4.0)).unwrap();
        let bbox2 = bbox.transform(matrix);
        //assert_eq!(bbox2.min,point(point(-1.4142, -1.7071, -1.7071).matrix)); //result:matrix: [[-1.4142135623730951], [-1.707106781186548], [-1.707106781186548], [1.0]]
        //assert_eq!(bbox2.max,point(1.4142, 1.7071, 1.7071)); //result: matrix: [[1.4142135623730951], [1.707106781186548], [1.707106781186548], [1.0]]

    }

    #[test]
    pub fn bounding_box_feature_202(){

        let g1 = Group::new();
        let mut s = Sphere::new();
        s.set_transform(translation(2.0,5.0,-3.0).dot(scale(2.0,2.0,2.0)).unwrap());
        let mut c = Cylinder::new();
        c.min = -2.0;
        c.max = 2.0;
        c.set_transform(translation(-4.0,-1.0,4.0).dot(scale(0.5,1.0,0.5)).unwrap());
 

     
        let mut g1_rc = wrap_this(g1);
        let c = wrap_this(c);                        
        let s = wrap_this(s);
        

        add_child(&mut g1_rc,&s); 
        //g1 <=> g2
        add_child(&mut g1_rc,&c); 

        let bbox = g1_rc.borrow_mut().bounding_box();
        assert_eq!(bbox,BoundingBox::new(point(-4.5, -3.0, -5.0) ,point(4.0, 7.0, 4.5)))

        

    }

    #[test]
    pub fn hexagon_feature(){
        let r = Ray::new(   point(0.0,0.0,0.0),
        vector(0.0,0.0,1.0));
    
        let c = hexagon();
        let c = take_members_out(&c);
        let xs = intersect(c,&r,vec![]);


    }


}


