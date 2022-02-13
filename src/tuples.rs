use std::{ops::{ Add, Sub, Neg, Mul}, vec};
use crate::Matrix;
#[derive(Debug, Clone,PartialEq)]
pub struct Element {
    pub matrix: Matrix
}

pub fn vector (e0:f64 , e1:f64 , e2:f64) -> Element{
    Element{
        matrix: Matrix::new(vec![vec![e0],vec![e1],vec![e2],vec![0.0_f64.clamp(0.0,1.0)]])
    }
}

pub fn point (e0:f64 , e1:f64 , e2:f64) -> Element{
    Element{
        matrix: Matrix::new(vec![vec![e0],vec![e1],vec![e2],vec![1.0_f64.clamp(0.0,1.0)]])
    }
}

impl Element {
    pub fn new(e0:f64, e1:f64 , e2:f64 , t:f64 ) -> Element{
        Element{
            matrix: Matrix::new(vec![vec![e0],vec![e1],vec![e2],vec![t.clamp(0.0,1.0)]])
        }
    }

    pub fn grabtype (&self) -> f64 {
        self.matrix.matrix[3][0]
    }
    pub fn grabloc (&self) -> Vec<f64> {
        let mut loc = vec![];
        for x in 0..(self.matrix.rows-1){
            loc.push(self.matrix.matrix[x][0])
        }
        loc
    }

    pub fn x (&self) -> f64 {
        self.matrix.matrix[0][0]
    }

    pub fn y (&self) -> f64 {
        self.matrix.matrix[1][0]
    }

    pub fn z (&self) -> f64 {
        self.matrix.matrix[2][0]
    }

    pub fn magnitude(&self) -> f64 {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2)).sqrt()
    }

    pub fn normal(&self) -> Element {
        let mag:f64 = self.magnitude();
        Element { 
            matrix: Matrix::new(vec![vec![self.x()*(1.0/mag)], vec![self.y() *(1.0/mag)], vec![self.z() *(1.0/mag)], vec![self.grabtype()*(1.0/mag).clamp(0.0,1.0)]])
           
        }
    }



    
    pub fn typecheck(&self) -> String {
        let thistype = self.grabtype() as u32;
        match thistype {
            1 => String::from("point"),
            2 => String::from("vector"),
            _ => String::from("error: not 0 or 1"),
        }

    }

    pub fn dot(&self, other: Element) -> f64{
        self.grabloc().iter().zip(other.grabloc().iter()).map(|(x, y)| x * y).sum()
    }

    pub fn cross(&self, other: Element) -> Element{
        Element { 
            matrix:  Matrix::new(vec![vec![self.y() * other.z() - self.z() * other.y()],
                                    vec![self.z() * other.x() - self.x() * other.z()],
                                    vec![self.x() * other.y() - self.y() * other.x()],
                                    vec![0.0]]) //cannot cross points
                        }
                    }
}

impl Sub for Element {
    type Output = Element;
    fn sub(self, other: Element) -> Element{
        Element {
            matrix: Matrix::new(vec![vec![self.x() - other.x()], 
            vec![self.y() - other.y()],
            vec![self.z() - other.z()],
            vec![(self.grabtype() - other.grabtype()).clamp(0.0,1.0)]])
            //self.loc.iter().zip(other.loc.iter()).map(|(x, y)| x - y).collect() -> cant collect into an array

        }
    }
}

impl Add for Element {
    type Output = Element;
    fn add(self, other: Element) -> Element{
        Element {
            matrix: Matrix::new(vec![vec![self.x() + other.x()], 
            vec![self.y() + other.y()],
            vec![self.z() + other.z()],
            vec![(self.grabtype() + other.grabtype()).clamp(0.0,1.0)]])
        }
    }
}

impl Neg for Element {
    type Output = Element;
    fn neg(self) -> Element {
        Element{
            matrix: Matrix::new( vec![vec![0.0 - self.x()], 
            vec![0.0 - self.y()], 
            vec![0.0 - self.z()],
            vec![(0.0 - self.grabtype()).clamp(0.0,1.0)]])
        }
    }
    
}

impl Mul<f64> for Element {
    type Output = Element;
    fn mul(self, other: f64) -> Element{
        Element {
            matrix: Matrix::new( vec![vec![self.x() * other], 
            vec![self.y() * other],
            vec![self.z() * other],
            vec![(self.grabtype() * other).clamp(0.0,1.0) ]])
            //vec![self.grabtype() * other]])
            // having the 0/1 indicator for type messes up the matrix, grows too much, we know that the type can only have 0 or 1 value
        }
    }
}


impl Mul<Element> for f64 {
    type Output = Element;
    fn mul(self, other: Element) -> Element{
        Element {
            matrix: Matrix::new(vec![vec![self * other.x()], 
            vec![self * other.y()],
            vec![self * other.z()],
            vec![ (other.grabtype() * self).clamp(0.0,1.0)]])
        }
    }
}



/*impl Index<usize> for Element {
    //usize is a valid index
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        &self.loc[index]
    }

}*/

