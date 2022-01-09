use std::ops::Index;

#[derive(Debug)]
pub struct Element {
    loc: [f64 ; 3], //elements
    iam: f64, //point or vector
}

pub fn Vector (e0:f64 , e1:f64 , e2:f64 , t:f64) -> Element{
    Element{
        loc : [ e0 , e1 , e2],
        iam : 0.0,
    }
}



impl Element {
    pub fn new(e0:f64 , e1:f64 , e2:f64 , t:f64 ) -> Element{
        Element{
            loc : [ e0 , e1 , e2],
            iam : t,
        }
    }

    pub fn grabtype (&self) -> f64 {
        self.iam
    }

    pub fn x (&self) -> f64 {
        self.loc[0]
    }

    pub fn y (&self) -> f64 {
        self.loc[1]
    }

    pub fn z (&self) -> f64 {
        self.loc[2]
    }
 
    pub fn typecheck(&self) -> String {
        let thistype = self.iam as u32;
        match thistype {
            1 => String::from("point"),
            2 => String::from("vector"),
            _ => String::from("error: not 0 or 1"),
        }

    }
}

impl Add for Element {
    type Output = Element;
    fn add(self, other: Element) -> Element{
        Element {
            loc: [self.x() + other.x(), self.y() + other.y(),self.z() + other.z()],
            iam: self.grabtype() + other.grabtype(),
        }
    }
}
impl Index<usize> for Element {
    //usize is a valid index
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        &self.loc[index]
    }

}




#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn ispoint(){
        let tester = Element::new(1.0,2.0,3.0,1.0);
        assert_eq!(1.0,tester.grabtype());
    }

    #[test]
    fn isvector(){
        let tester = Element::new(1.0,2.0,3.0,0.0);
        assert_eq!(0.0,tester.grabtype());
    }  
}


/*struct proj{

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
*/


