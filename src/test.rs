
#[cfg(test)]
pub mod tests{
    
    use crate::Canvas;
    use crate::rays::{Ray, hit, intersect, Intersections,Intersection};
    use crate::{Element, vector, point};
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
            let mut p = Element { matrix: point(1.0,0.0,0.0).matrix.rotate_z(i as f32 * PI/6.0)};
            //println!("initial {:?}" , p);
            p = p * (r);
            //println!("times radius {:?}" , p);
            p = Element {matrix:  p.matrix.translation(100.0,100.0,0.0)};
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
        let s = point(0.0,0.0,0.0);
        let i1 = Intersection::new(3.5,&s);
        assert_eq!(3.5, i1.t);
        assert_eq!(&s, i1.o);

        let s = point(0.0,0.0,0.0);
        let i1 = Intersection::new(1.0,&s);
        let i2 = Intersection::new(2.0,&s);
        let group = Intersections::new(vec![i1,i2]);
        assert_eq!(2, group.count);
        assert_eq!(1.0, group.h[0].t);
        assert_eq!(2.0, group.h[1].t);
    }

    #[test]
    fn sphere_features_2(){
        let r1 = Ray::new(point(0.0,0.0,-5.0), vector(0.0,0.0,1.0));
        let s1 = point(0.0,0.0,0.0,);
        let xs = intersect(&r1,&s1);
        assert_eq!(2,xs.count);
        assert_eq!(&s1,xs.h[0].o);
        assert_eq!(&s1,xs.h[1].o);
    }

    #[test]
    fn intersection_feature_hits(){
        let s = point(0.0,0.0,0.0,);

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

    