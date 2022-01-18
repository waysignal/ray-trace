use std::ops::{Index, Add, Sub, Neg, Mul, Div};

#[derive(Debug, Clone, Copy)]
pub struct Element {
    loc: [f32 ; 3], //elements
    iam: f32, //point or vector
}

pub fn vector (e0:f32 , e1:f32 , e2:f32) -> Element{
    Element{
        loc : [ e0 , e1 , e2],
        iam : 0.0,
    }
}

pub fn point (e0:f32 , e1:f32 , e2:f32) -> Element{
    Element{
        loc : [ e0 , e1 , e2],
        iam : 1.0,
    }
}

impl Element {
    pub fn new(e0:f32, e1:f32 , e2:f32 , t:f32 ) -> Element{
        Element{
            loc : [ e0 , e1 , e2],
            iam : t,
        }
    }

    pub fn grabtype (&self) -> f32 {
        self.iam
    }
    pub fn grabloc (&self) -> [f32;3] {
        self.loc
    }

    pub fn x (&self) -> f32 {
        self.loc[0]
    }

    pub fn y (&self) -> f32 {
        self.loc[1]
    }

    pub fn z (&self) -> f32 {
        self.loc[2]
    }

    pub fn magnitude(&self) -> f32 {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2)).sqrt()
    }

    pub fn normal(&self) -> Element {
        let mag:f32 = self.magnitude();
        Element { 
            loc: [self.x()*(1.0/mag), self.y() *(1.0/mag), self.z() *(1.0/mag)], 
            iam: self.grabtype()*(1.0/mag) 
        }
    }



    
    pub fn typecheck(&self) -> String {
        let thistype = self.iam as u32;
        match thistype {
            1 => String::from("point"),
            2 => String::from("vector"),
            _ => String::from("error: not 0 or 1"),
        }

    }

    pub fn dot(&self, other: Element) -> f32{
        self.loc.iter().zip(other.loc.iter()).map(|(x, y)| x * y).sum()
    }

    pub fn cross(&self, other: Element) -> Element{
        Element { 
            loc: [self.y() * other.z() - self.z() * other.y(),
                  self.z() * other.x() - self.x() * other.z(),
                  self.x() * other.y() - self.y() * other.x()] ,
            iam: 0.0 }
    }
}

impl Sub for Element {
    type Output = Element;
    fn sub(self, other: Element) -> Element{
        Element {
            loc: [self.x() - other.x(), self.y() - other.y(),self.z() - other.z()],
            //self.loc.iter().zip(other.loc.iter()).map(|(x, y)| x - y).collect() -> cant collect into an array

            iam: self.grabtype() - other.grabtype(),
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

impl Neg for Element {
    type Output = Element;
    fn neg(self) -> Element {
        Element{
            loc: [0.0 - self.x(), 0.0 - self.y(), 0.0 - self.z()],
            iam: 0.0 - self.grabtype()
        }
    }
    
}

impl Mul<f32> for Element {
    type Output = Element;
    fn mul(self, other: f32) -> Element{
        Element {
            loc: [self.x() * other, self.y() * other, self.z() * other],
            iam: self.grabtype() * other,
        }
    }
}


impl Mul<Element> for f32 {
    type Output = Element;
    fn mul(self, other: Element) -> Element{
        Element {
            loc: [self * other.x(), self * other.y(), self * other.z()],
            iam: other.grabtype() * self,
        }
    }
}



impl Index<usize> for Element {
    //usize is a valid index
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        &self.loc[index]
    }

}




#[cfg(test)]
mod tests{
    use super::*;

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
}