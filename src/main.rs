mod tuples;

use tuples::{Element, vector, point};

static EPSILON: f64 = 0.00001;
fn main() {
    let test1 =  point(-1.0,2.0,3.0);
    let test2=  point(0.0,2.0,3.0);
    println!("x:{} y:{} z:{} type:{}" , test1.x(),test1.y(),test1.z(),test1.typecheck());
    println!( "{:?}", test2.cross(test1) );

}


fn equal_floats(a:f64,b:f64) -> bool {
    if (a-b) < EPSILON{
        true   
    } else {
        false
    }
}



#[cfg(test)]
mod tests{
  use super::*; 
  #[test]
  fn main() {
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
                            v: vector(1.0,1.0,0.0).normal(),};
    while proj1.p.y() > 0.0 {
            proj1 = tick(&env1,proj1);
            eprintln!( "two {:?}", proj1.p);
    
    }
}
}
        



