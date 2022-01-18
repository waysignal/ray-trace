mod tuples;
mod canvas;
mod matrix;
mod transformation;
use std::{f32::consts::PI, ops::Add};

use tuples::{Element, vector, point};
use canvas::{Canvas};
use matrix::matrix::Matrix;
//use transformation;




fn main() {
    // let test1 =  point(-1.0,2.0,3.0);
    // let test2=  point(0.0,2.0,3.0);
    // //println!("x:{} y:{} z:{} type:{}" , test1.x(),test1.y(),test1.z(),test1.typecheck());
    // //println!( "{:?}", test2.cross(test1) );


    // let x = Matrix::new( vec![vec![1.0,2.0,3.0];3]);
    // let y = Matrix::new(vec![vec![3.0];3]);
    // let p = x.identity();
    // let p = p.transpose();

    //3x3
    //let deter = Matrix::new(vec![vec![1.0,2.0,6.0],vec![-5.0,8.0,-4.0], vec![2.0,6.0,4.0]]);
    
    //4x4
    // let deter = Matrix::new(vec![vec![-2.0,-8.0,3.0,5.0],
    //                                         vec![-3.0,1.0,7.0,3.0], 
    //                                         vec![1.0,2.0,-9.0,6.0],
    //                                         vec![-6.0,7.0,7.0,-9.0]]);

    // 4x4 invertible
    // let a = Matrix::new(vec![vec![-5.0,2.0,6.0,-8.0],
    //                                         vec![1.0,-5.0,1.0,8.0], 
    //                                         vec![7.0,7.0,-6.0,-7.0],
    //                                         vec![1.0,-3.0,7.0,4.0]]);

    // // 4x4 invertible 2
    // let b = Matrix::new(vec![vec![9.0,3.0,0.0,9.0],
    //                                         vec![-5.0,-2.0,-6.0,-3.0], 
    //                                         vec![-4.0,9.0,6.0,4.0],
    //                                         vec![-7.0,6.0,6.0,2.0]]);

    //4x4 uninvertible
    // let deter = Matrix::new(vec![vec![-5.0,2.0,6.0,-8.0],
    //                                         vec![1.0,-5.0,1.0,8.0], 
    //                                         vec![7.0,7.0,-6.0,-7.0],
    //                                         vec![0.0,0.0,0.0,0.0]]);
    
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

    let mut clock = Canvas::new(200,200);
    let r = 3.0/8.0 * 200.0;
    let mut points = vec![];
    for i in 0..12{
        let mut p = Element { matrix: point(1.0,0.0,0.0).matrix.rotate_z(i as f32 * PI/6.0)};
        //println!("initial {:?}" , p);
        p = p * (r);
        //println!("times radius {:?}" , p);
        p = Element {matrix:  p.matrix.translation(100.0,100.0,0.0)};
        //println!("final {:?}" , p);
        points.push(p)
    }

    for (x, y)  in points.iter().enumerate(){
        clock.color(y.x() as usize,200 - y.y() as usize,vector(255.0,255.0,255.0))
    }

    clock.draw()

}





#[cfg(test)]
mod tests{
  use super::*; 
  #[test]
  fn main() {
    let mut cannon = Canvas::new(900,550);
    // struct proj{
    //     pub p: Element,
    //     pub v: Element,
    //     } 

    // struct envi{
    //     pub g: Element,
    //     pub w: Element,
    //     } 
    // fn tick(a:&envi, b:proj) -> proj {
    //         proj { p: b.p + b.v, 
    //     v: b.v + a.g + a.w }
    //     }
    
    // let env1 = envi {  g: vector(0.0,-0.1,0.0), 
    //                         w: vector(-0.01,0.0,0.0),};
    // let mut proj1 = proj {  p: point(0.0,1.0,0.0),
    //                         v: vector(1.0,1.8,0.0).normal() * 11.25,};
    
    // while proj1.p.y() > 0.0 {
    //         eprintln!( "two {:?}", proj1.p);
    //         cannon.color((proj1.p.grabloc()[0] -1.0)  as usize, 
    //         (cannon.h as f32- proj1.p.grabloc()[0] -1.0) as usize, vector(1.0,1.0,1.0));
    //         proj1 = tick(&env1,proj1);
    // }
    cannon.draw();
}
  #[test]
  fn main2() {


 
}
}
        



//////////
/*
let mut cannon = Canvas::new(900,550);
    struct proj{
        pub p: Element,
        pub v: Element,
        } 

    struct envi{
        pub g: Element,
        pub w: Element,
        } 
    fn tick(a:&envi, b:proj) -> proj {
            proj { p: b.p + b.v, 
        v: b.v + a.g + a.w }
        }
    
    let env1 = envi {  g: vector(0.0,-0.1,0.0), 
                            w: vector(-0.01,0.0,0.0),};
    let mut proj1 = proj {  p: point(0.0,1.0,0.0),
                            v: vector(1.0,1.8,0.0).normal() * 11.25,};
    
    while proj1.p.y() > 0.0 {
            eprintln!( "two {:?}", proj1.p);
            
            cannon.color((proj1.p.x() -1.0 )  as usize, 
                            (cannon.h() as f32- proj1.p.y() - 1.0) as usize, vector(255.0,255.0,255.0));
               
            proj1 = tick(&env1,proj1);

         
            }
        
    cannon.draw();

    */