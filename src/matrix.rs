

// mod operations{
//     use crate::{Element, Matrix, point, vector};
//     pub trait Operations
//     {
//         fn translation(&self,x:f64,y:f64,z:f64) -> Self;
//         //looking at trait objects to be able to call translation on both Ray and Matrix without duplicating code
//         //cannot have functions that return different things, cannot be resolved dynamically?
//         //sike dont actually need this whole block
//     }

//     struct TEMP<T> {
//         m: T
//     }

//     impl<T> TEMP<T> {
//         fn translation(&self,x:f64,y:f64,z:f64) -> T{

//         }    
//     }
// }





pub mod matrix{
    use std::ops::{Neg,Div};
    static EPSILON: f64 = 0.00001;




    #[derive(Debug, Clone,PartialEq)]
    pub struct Matrix{
        pub matrix:  Vec<Vec<f64>>, //array on stack will be dropped anyways
        pub rows: usize,                            // pub bc want to access anyways
        pub cols: usize,
        //changed this from u64 to usize to save
    }

    pub fn equal_floats(a:&f64,b:&f64) -> bool {
        if (a.abs()-b.abs()) < EPSILON{
            true   
        } else {
            false
        }
    }


    pub fn translation(x:f64,y:f64,z:f64) -> Matrix {
        let mut transform = Matrix::zero(4,4).identity();
        transform.matrix[0][3] = x;
        transform.matrix[1][3] = y;
        transform.matrix[2][3] = z;
        transform
    }

    pub fn scale(x:f64,y:f64,z:f64) -> Matrix{
        let mut transform = Matrix::zero(4,4).identity();
        transform.matrix[0][0] = x;
        transform.matrix[1][1] = y;
        transform.matrix[2][2] = z;
        transform
        //transform.dot(self).unwrap()  
    }

    pub fn rotate_x(rad: f64) -> Matrix{
        let mut transform = Matrix::zero(4,4).identity();
        transform.matrix[1][1] = rad.cos();
        transform.matrix[1][2] = -rad.sin();
        transform.matrix[2][1] = rad.sin();
        transform.matrix[2][2] = rad.cos();
        transform  
    }

    pub fn rotate_y(rad: f64) -> Matrix{
        let mut transform = Matrix::zero(4,4).identity();
        transform.matrix[0][0] = rad.cos();
        transform.matrix[0][2] = rad.sin();
        transform.matrix[2][0] = -rad.sin();
        transform.matrix[2][2] = rad.cos();
        transform  
    }

    pub fn rotate_z(rad: f64) -> Matrix{
        let mut transform = Matrix::zero(4,4).identity();
        transform.matrix[0][0] = rad.cos();
        transform.matrix[0][1] = -rad.sin();
        transform.matrix[1][0] = rad.sin();
        transform.matrix[1][1] = rad.cos();
        transform
    }

    pub fn shearing(xy:f64,xz:f64,yx:f64,yz:f64,zx:f64,zy:f64) -> Matrix{
        let mut transform = Matrix::zero(4,4).identity();
        transform.matrix[0][1] = xy;
        transform.matrix[0][2] = xz;
        transform.matrix[1][0] = yx;
        transform.matrix[1][2] = yz;
        transform.matrix[2][0] = zx;
        transform.matrix[2][1] = zy;
        transform  
    }

    impl Matrix{

        pub fn new(mat: Vec<Vec<f64>>) -> Matrix{
            let m = mat.len();
            let n = mat[0].len();
            Matrix{
                matrix: mat, 
                rows: m,// row x column
                cols: n,
                
            
            }
        }

        pub fn update(self) -> Matrix{
            let m = self.matrix.len();
            let n = self.matrix[0].len();
            Matrix { matrix: self.matrix, rows:m, cols: n }
        }

        pub fn zero(m: usize, n: usize) -> Matrix{
        
            Matrix{
                matrix: vec![vec![0.0;n];m], 
                rows: m,// row x column
                cols: n,
            
            }
        }

        pub fn get(&self, m: usize, n: usize) -> f64{
            self.matrix[m][n] 
            // go into mth row and then nth column
        }

        pub fn equal(&self, other: Matrix) -> bool {
            let a_rows = self.rows;
            let a_cols = self.cols;
            let b_rows = other.rows;
            let b_cols = other.cols;
            let mut cond = true;
            if a_rows == b_rows && a_cols == b_cols && cond == true {
                for (i,j) in (&self.matrix).iter().enumerate() {
                    let a = &self.matrix[i];
                    let b = &other.matrix[i];

                    for (a_e, b_e) in a.iter().zip(b){
                        if equal_floats(a_e ,b_e) == false {
                            cond = false;
                        }
                        else{
                            continue;
                        }
                    }

                } 

            } else {
                cond = false;
            }
            cond
        }

        pub fn transpose(self) -> Matrix{
            let mut result = Matrix::zero(self.matrix[0].len(), self.matrix.len());
            for a in 0..result.rows {
                for b in 0..result.cols{
                    result.matrix[a][b] = self.get(b,a)
                }
            }
            result
        }

        pub fn dot(&self, other: Matrix) -> Option<Matrix> {
            let a_rows = self.rows;
            let b_cols = other.cols;
            let a_cols = self.cols;
            let b_rows = other.rows;
            if a_cols != b_rows {
                None
            }  else {                                                                                                                      
                let mut product = Matrix::zero(a_rows,b_cols);
                let other = other.transpose();
                for x in 0..product.rows{
                    for y in 0..product.cols{
                        product.matrix[x][y] = self.matrix[x].iter().zip(other.matrix[y].iter()).map(|(x, y)| x * y).sum()
                    }

                }

            Some(product)
            }
        }

        pub fn identity(&self) -> Matrix{
            let mut place = Matrix::zero(self.rows, self.cols);
            for m in 0..place.rows{
                for n in 0..place.cols{
                    if m == n {
                        place.matrix[m][n] = 1.0;
                    }
                }
            }
            place
        }

        pub fn det(&self) -> Option<f64>{
            let mut deter_f = 0.0;
            if (self.rows != self.cols) || self.rows == (0 as usize) || self.rows == (1 as usize) {
                None
            } else {
                if self.rows == 2{
                    let a = self.matrix[0][0];
                    let b = self.matrix[0][1];
                    let c = self.matrix[1][0];
                    let d = self.matrix[1][1];
        
                    Some(a*d - b*c)
                } else {
                    //for loop through first row, cofactor it
                    for (x,y) in self.matrix[0].iter().enumerate(){
                        deter_f += self.cofactor(0, x) * y;
                    }
                    Some(deter_f)
                }
                
            }
        }

        pub fn submatrix(&self, row: usize, col: usize) -> Matrix{
            let mut subm = Matrix::new(self.matrix.clone());
            subm.matrix.remove(row);
            subm = subm.transpose();
            subm.matrix.remove(col);
            subm.transpose() //transpose auto updates
        }

        pub fn minor(&self, row: usize, col: usize) -> f64{
            let subm = self.submatrix(row, col);
            subm.det().unwrap()
        }

        pub fn cofactor(&self, row: usize, col: usize) -> f64{
            if let 0 = (row  + col)%2 {
                self.minor(row,col) 
            } else {
                self.minor(row,col).neg()
            }
        }
        
        pub fn invert(&self) -> Option<Matrix>{
            if self.det().unwrap() == 0.0 {
                None
            } else{
                let mut new = Matrix::zero(self.rows, self.rows);
                for m in 0..self.rows{
                    for n in 0..self.cols{   
                        new.matrix[m][n] = self.cofactor(m, n);
                
                    }
                }
                new = new.transpose();
                new = new / self.det().unwrap();
                Some(new)
            }
        }



    

    
        
    }

    impl Div<f64> for Matrix{
        type Output = Matrix;
        fn div(mut self, other: f64) -> Matrix{
            for m in 0..self.rows{
                for n in 0..self.cols{ 
                    self.matrix[m][n] = self.matrix[m][n]/other;
                }
            }
            self
        

        }
    }
}