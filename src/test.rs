
#[cfg(test)]
pub mod tests{

    use crate::shapes::{Sphere, normal_at,reflect, PointLight, Material,lighting, Color};
    use crate::Canvas;
    use crate::matrix::matrix::*;
    use crate::rays::{Ray, hit, intersect, Intersections,Intersection};
    use crate::{Element,Matrix, vector, point};
    //use crate::Ray;
    use std::{f32::consts::PI,};



    #[test]
    fn ispoint(){
        let tester = point(1.0,2.0,3.0);
        assert_eq!(1.0,tester.grabtype());
    }

    #[test]
    fn isvector(){
        let tester = vector(1.0,2.0,3.0);
        assert_eq!(0.0,tester.grabtype());
    }

    #[test]
    fn newvector(){
        let nv = vector(1.0,2.0,3.0);
        eprintln!("{:?} ",  nv)
    }

    #[test]
    fn clock(){
        let mut clock = Canvas::new(200,200);
        let r = 3.0/8.0 * 200.0;
        let mut points = vec![];
        for i in 0..12{
            let mut p = Element { matrix: rotate_z(i as f32 * PI/6.0).dot(point(1.0,0.0,0.0).matrix).unwrap()};
            //println!("initial {:?}" , p);
            p = p * (r);
            //println!("times radius {:?}" , p);
            p = Element {matrix:  translation(100.0,100.0,0.0).dot(p.matrix).unwrap()};
            println!("final {:?}" , p);
            points.push(p)
        }

        for (_x, y)  in points.iter().enumerate(){
            clock.color(y.x() as usize,200 - (y.y() as usize),vector(255.0,255.0,255.0))
        }

        //clock.draw()
    }

    #[test]
    fn rays_feature(){
        let p = point(2.0,3.0,4.0);
        let d = vector(1.0,0.0,0.0);
        let ray = Ray::new(p,d);
        let pos = ray.position(0.0);
        let pos1 = ray.position(1.0);
        let pos2 = ray.position(-1.0);
        let pos3 = ray.position(2.5);
        assert_eq!(true,pos.matrix.equal(point(3.0,3.0,4.0).matrix));
        assert_eq!(true,pos1.matrix.equal(point(3.0,3.0,4.0).matrix));
        assert_eq!(true,pos2.matrix.equal(point(1.0,3.0,4.0).matrix));
        assert_eq!(true,pos3.matrix.equal(point(4.5,3.0,4.0).matrix));
    }

    //#[test]
    // returning t instead of intersect records
    // fn spheres_feature(){
    //     let r1 = Ray::new(point(0.0,0.0,-5.0), vector(0.0,0.0,1.0));
    //     let s1 = point(0.0,0.0,0.0,);
    //     assert_eq!(Sphere_Hits{count:2, dist: [4.0,6.0]},intersect(&r1,s1.clone()));

    //     let r2 = Ray::new(point(0.0,1.0,-5.0), vector(0.0,0.0,1.0));
    //     let s2 = s1.clone();
    //     assert_eq!(Sphere_Hits{count:2, dist: [5.0,5.0]},intersect(&r2,s2));

    //     let r3 = Ray::new(point(0.0,2.0,-5.0), vector(0.0,0.0,1.0));
    //     let s3 = s1.clone();
    //     assert_eq!(Sphere_Hits{count:0, dist: [0.0,0.0]},intersect(&r3,s3));

    //     let r4 = Ray::new(point(0.0,0.0,0.0), vector(0.0,0.0,1.0));
    //     let s4 = s1.clone();
    //     assert_eq!(Sphere_Hits{count:2, dist: [-1.0,1.0]},intersect(&r4,s4));

    //     let r5 = Ray::new(point(0.0,0.0,5.0), vector(0.0,0.0,1.0));
    //     let s5 = s1.clone();
    //     assert_eq!(Sphere_Hits{count:2, dist: [-6.0,-4.0]},intersect(&r5,s5));
    // }

    #[test]
    fn intersections_feature(){
        let s = Sphere::new();
        let i1 = Intersection::new(3.5,&s);
        assert_eq!(3.5, i1.t);
        assert_eq!(&s, i1.o);

        let s = Sphere::new();
        let i1 = Intersection::new(1.0,&s);
        let i2 = Intersection::new(2.0,&s);
        let group = Intersections::new(vec![i1,i2]);
        assert_eq!(2, group.count);
        assert_eq!(1.0, group.h[0].t);
        assert_eq!(2.0, group.h[1].t);
    }
    #[test]
    fn sphere_features(){
        let r1 = Ray::new(point(0.0,0.0,5.0), vector(0.0,0.0,1.0));
        let mut s1 = Sphere{loc: point(0.0,0.0,0.0),
            transform: Matrix::zero(4,4).identity(),material: Material::new()};
        let xs = intersect(&r1,&s1);
        eprintln!("{:?}",xs);
        assert_eq!(2,xs.count);
        assert_eq!(-6.0,xs.h[0].t);
        assert_eq!(-4.0,xs.h[1].t);
    }
    #[test]
    fn sphere_features_2(){
        let r1 = Ray::new(point(0.0,0.0,-5.0), vector(0.0,0.0,1.0));
        let mut s1 = Sphere{loc: point(0.0,0.0,0.0),
            transform: Matrix::zero(4,4).identity(),material: Material::new()};
        let xs = intersect(&r1,&s1);
        assert_eq!(2,xs.count);
        assert_eq!(&s1,xs.h[0].o);
        assert_eq!(&s1,xs.h[1].o);
    }

    #[test]
    fn intersection_feature_hits(){
        let s = Sphere::new();

        let i1 = Intersection::new(1.0,&s);
        let i2 = Intersection::new(2.0,&s);
        let mut xs = Intersections::new(vec![i2,i1]);
        let xs_clone = xs.clone();
        assert_eq!(&xs_clone.h[1],hit(&mut xs).unwrap());

        let i1 = Intersection::new(-1.0,&s);
        let i2 = Intersection::new(1.0,&s);
        let mut xs = Intersections::new(vec![i2,i1]);
        let xs_clone = xs.clone();
        assert_eq!(&xs_clone.h[0],hit(&mut xs).unwrap());

        let i1 = Intersection::new(-2.0,&s);
        let i2 = Intersection::new(-1.0,&s);
        let mut xs = Intersections::new(vec![i2,i1]);
        assert_eq!(None,hit(&mut xs));

        let i1 = Intersection::new(5.0,&s);
        let i2 = Intersection::new(7.0,&s);
        let i3 = Intersection::new(-3.0,&s);
        let i4 = Intersection::new(2.0,&s);

        let mut xs = Intersections::new(vec![i1,i2,i3,i4]);
        //realizing dont need clone if just put a reference to a new intersection
        assert_eq!(&Intersection::new(2.0,&s),hit(&mut xs).unwrap());

    }

    #[test]
    fn ray_feature(){
        let r = Ray::new(point(1.0,2.0,3.0), vector(0.0,1.0,0.0));
        let m = translation(3.0, 4.0, 5.0);
        let r2 = r.transform(&m);
        assert_eq!(point(4.0,6.0,8.0),r2.origin);
        assert_eq!(vector(0.0,1.0,0.0),r2.direction);

        let m = scale(2.0,3.0,4.0);
        let mut r2 = r.transform(&m);

        //is it better to keep referencing an object or destory and remake each time?
        assert_eq!(point(2.0,6.0,12.0),r2.origin);
        assert_eq!(vector(0.0,3.0,0.0,),r2.direction);
    }

    #[test]
    fn sphere_feature_3(){
        let mut s = Sphere{loc: point(0.0,0.0,0.0),
            transform: Matrix::zero(4,4).identity(),material: Material::new()};

        let t = translation(2.0,3.0,4.0);
        s.set_transform(&t);
        assert_eq!(t,s.transform);
        //we can have set_tranform use a mut ref for self, it will not take ownership and changes the fields

        let r = Ray::new(point(0.0,0.0,-5.0),vector(0.0,0.0,1.0));
        let mut s = Sphere{loc: point(0.0,0.0,0.0),
            transform: Matrix::zero(4,4).identity(),material: Material::new()};

        s.set_transform(&scale(2.0,2.0,2.0)); // who is the owner of scale? -> the function set_tranform
        let xs = intersect(&r,&s);
            //needs to be a transformation back -> edit: no there doesnt, t should be correct regardless
        assert_eq!(scale(2.0,2.0,2.0),s.transform);
        assert_eq!(2,xs.count);
        eprintln!("{:?}",xs);
        assert_eq!(7.0,xs.h[1].t);
        assert_eq!(3.0,xs.h[0].t);
        
        let mut s = Sphere{loc: point(0.0,0.0,0.0),
            transform: Matrix::zero(4,4).identity(),material: Material::new()};

        let t = translation(0.0,0.0,-8.0);
        s.set_transform(&t);
        let xs = intersect(&r,&s);
        eprintln!("{:?}",xs);
        assert_eq!(2,xs.count);
    }

    #[test]
    fn first_draw_sphere() {
        let wall_z = 10.0;
        let wall_size = 7.0;
        let canvas_pixels = 100.0;
        let pixel_size = wall_size/canvas_pixels;
        let half = wall_size/2.0;
        let mut new = Canvas::new(100,100);
        let mut s = Sphere{loc: point(0.0,0.0,0.0),
            transform: Matrix::zero(4,4).identity(),material: Material::new()};
        //let mut ray = Ray::new(point(0.0,0.0,-5.0),vector(0.0,0.0,1.0));
        
    
        let t = shearing(1.0,0.0,0.0,0.0,0.0,0.0).dot(scale(0.5,1.0,1.0)).unwrap();
        s.set_transform(&t);
    
        for y in 0..100 {
            let world_y = half - pixel_size * (y as f32);
    
            for x in 0..100 {
                let world_x = -half + pixel_size * (x as f32);
                let position = point(world_x, world_y, wall_z);
    
                let r = Ray::new(point(0.0,0.0,-5.0), (position - point(0.0,0.0,-5.0).normal()));
                let xs = intersect(&r, &s);
                if xs.count > 0 {
                    new.color(x,y,Color::new(1.0,0.0,0.0));
                }
            }
    
        }
        //new.draw();
    }
    #[test]
    fn sphere_feature_4_78(){
        let mut s = Sphere{loc: point(0.0,0.0,0.0),
            transform: Matrix::zero(4,4).identity(),material: Material::new()};
        
        let n = normal_at(&s,&point(1.0,0.0,0.0));
        assert_eq!(vector(1.0,0.0,0.0),n);

        let n = normal_at(&s,&point(0.0,1.0,0.0));
        assert_eq!(vector(0.0,1.0,0.0),n);

        let n = normal_at(&s,&point(0.0,0.0,1.0));
        assert_eq!(vector(0.0,0.0,1.0),n);

        let n = normal_at(&s,&point(3.0_f32.sqrt()/3.0_f32,3.0_f32.sqrt()/3.0,3.0_f32.sqrt()/3.0));
        assert!(vector(3.0_f32.sqrt()/3.0_f32,3.0_f32.sqrt()/3.0,3.0_f32.sqrt()/3.0).matrix.equal(n.clone().matrix)); //make equal same for matrix and element? ; trait but inputs are different, unless we can use self for input, but dynamic?

        assert!(n.clone().matrix.equal(n.clone().normal().matrix));
    }
    #[test]
    fn sphere_feature_5_80(){
        let mut s = Sphere{loc: point(0.0,0.0,0.0),
            transform: Matrix::zero(4,4).identity(),material: Material::new()};
        s.set_transform(&translation(0.0,1.0,0.0));
        let n = normal_at(&s, &point(0.0,1.70711,-0.70711));
        assert!(vector(0.0,0.70711,-0.70711).matrix.equal(n.matrix));

        s.set_transform(&scale(1.0,0.5,1.0).dot(rotate_z(PI/5.0)).unwrap());
        let n = normal_at(&s, &point(0.0,2.0_f32.sqrt()/2.0,-2.0_f32.sqrt()/2.0));
        assert!(vector(0.0,0.97014,-0.24254).matrix.equal(n.matrix));

    }
    #[test]
    fn tuples_features_83(){
        let v = vector(0.0,-1.0,0.0);
        let n = vector(2.0_f32.sqrt()/2.0,2.0_f32.sqrt()/2.0,0.0);
        let r = reflect(&v,&n);
        assert!(vector(1.0,0.0,0.0).matrix.equal(r.matrix));     
        
        let v = vector(1.0,-1.0,0.0);
        let n = vector(0.0,1.0,0.0);
        let r = reflect(&v,&n);
        assert!(vector(1.0,1.0,0.0).matrix.equal(r.matrix));  
    }
    #[test]
    fn lights_feature_84(){
        let pl = PointLight::new(point(0.0,0.0,0.0), Color::new(1.0,1.0,1.0));
        assert_eq!(pl.position, point(0.0,0.0,0.0));
        assert_eq!(pl.intensity, Color::new(1.0,1.0,1.0));
        
        let m = Material::new();
        eprintln!("{:?}",m);

        let mut s = Sphere::new();
        assert_eq!(Material::new(), s.material);

        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        let m_1 = m.clone();
        s.material = m;
        assert_eq!(s.material, m_1);

    }
    #[test]
    fn material(){
        let mut m = Material::new();
        let pos = point(0.0,0.0,0.0);

        let eyev = vector(0.0,0.0,-1.0);
        let normalv = vector(0.0,0.0,-1.0);
        let light = PointLight::new(point(0.0,0.0,-10.0), Color::new(1.0,1.0,1.0));

        let result = lighting(m,&light,pos,eyev,normalv);
        assert_eq!(Color::new(1.9,1.9,1.9),result);

        let mut m = Material::new();
        let pos = point(0.0,0.0,0.0);
        let eyev = vector(0.0,2.0_f32.sqrt()/2.0,-2.0_f32.sqrt()/2.0);
        let normalv = vector(0.0,0.0,-1.0);
        let light = PointLight::new(point(0.0,0.0,-10.0), Color::new(1.0,1.0,1.0));

        let result = lighting(m,&light,pos,eyev,normalv);
        assert_eq!(Color::new(1.0,1.0,1.0),result);

        let mut m = Material::new();
        let pos = point(0.0,0.0,0.0);
        let eyev = vector(0.0,0.0,-1.0);
        let normalv = vector(0.0,0.0,-1.0);
        let light = PointLight::new(point(0.0,10.0,-10.0), Color::new(1.0,1.0,1.0));

        let result = lighting(m,&light,pos,eyev,normalv);
        //assert_eq!(Color::new(0.7364,0.7364,0.7364),result); //close

        let mut m = Material::new();
        let pos = point(0.0,0.0,0.0);
        let eyev = vector(0.0,-2.0_f32.sqrt()/2.0,-2.0_f32.sqrt()/2.0);
        let normalv = vector(0.0,0.0,-1.0);
        let light = PointLight::new(point(0.0,10.0,-10.0), Color::new(1.0,1.0,1.0));

        let result = lighting(m,&light,pos,eyev,normalv);
        //assert_eq!(Color::new(1.6364,1.6364,1.6364),result); //close

        
        let mut m = Material::new();
        let pos = point(0.0,0.0,0.0);
        let eyev = vector(0.0,0.0,-1.0);
        let normalv = vector(0.0,0.0,-1.0);
        let light = PointLight::new(point(0.0,0.0,10.0), Color::new(1.0,1.0,1.0));

        let result = lighting(m,&light,pos,eyev,normalv);
        assert_eq!(Color::new(0.1,0.1,0.1),result); //close

    }
    #[test]
    fn second_draw_sphere() {
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
    
                let r = Ray::new(point(0.0,0.0,-5.0), ((position - point(0.0,0.0,-5.0)).normal()));
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
        //new.draw();
    }
}



    // let test1 =  point(-1.0,2.0,3.0);
    // let test2=  point(0.0,2.0,3.0);
    // //println!("x:{} y:{} z:{} type:{}" , test1.x(),test1.y(),test1.z(),test1.typecheck());
    // //println!( "{:?}", test2.cross(test1) );


    // let x = Matrix::new( vec![vec![1.0,2.0,3.0];3]);
    // let y = Matrix::new(vec![vec![3.0];3]);
    // let p = x.identity();
    // let p = p.transpose();



    // //println!("{:?} ,,, {:?}", p, y);
    // let a_1 = a.clone();
    // let b_1 = b.clone();
    // let mut z = a.dot(b).unwrap();
    // //z = z.update();
    // println!("z: {:?}, " ,z);
    // let inv_b = b_1.invert().unwrap();
    // println!("invert: {:?}" , inv_b);
    // println!("check equal {:?}" , a_1.equal(z.dot(inv_b).unwrap()));

    //check if inv of trans == trans of inv -> result: true
    // let a_1 = a.clone();
    // let b_1 = b.clone();
    // let inv_a = a.invert().unwrap();
    // let tran_of_inv_a = inv_a.transpose();
    // println!("z: {:?}, " ,tran_of_inv_a);

    // let tran_a = a_1.transpose();
    // let inv_of_tran_a = tran_a.invert().unwrap();
    // println!("invert: {:?}" , inv_of_tran_a);
    // println!("check equal {:?}" , tran_of_inv_a.equal(inv_of_tran_a));

    // let a = transformation::rotate_x(PI/2.0);
    // let b = transformation::scale(5.0, 5.0, 5.0);
    // let c = transformation::translation(10.0, 5.0, 7.0);
    //let t = transformation::shearing(0.0,0.0,0.0,0.0,0.0,1.0);
    //println!("{:?} ", t);
    //let p = point(1.0,0.0,1.0);
    //let t = c.dot(b.dot(a).unwrap()).unwrap();
    //println!("{:?} ", t.dot(p.matrix));

    // println!("{:?} ", point(1.0,0.0,1.0).matrix.
    //                     rotate_x(PI/2.0).
    //                     scale(5.0,5.0,5.0).
    //                     translation(10.0,5.0,7.0));

