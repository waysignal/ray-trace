
/*Moving into matrix methods, might be a pain to deal with having a separate struct for elements and matrix,  */
// impl Matrix{
//     pub fn translation(self,x:f32,y:f32,z:f32) -> Matrix {
//         let mut transform = Matrix::zero(4,4).identity();
//         transform.matrix[0][3] = x;
//         transform.matrix[1][3] = y;
//         transform.matrix[2][3] = z;
//         transform.dot(self).unwrap()
//     }

//     pub fn scale(self,x:f32,y:f32,z:f32) -> Matrix{
//         let mut transform = Matrix::zero(4,4).identity();
//         transform.matrix[0][0] = x;
//         transform.matrix[1][1] = y;
//         transform.matrix[2][2] = z;
//         transform.dot(self).unwrap()  
//     }

//     pub fn rotate_x(self,rad: f32) -> Matrix{
//         let mut transform = Matrix::zero(4,4).identity();
//         transform.matrix[1][1] = rad.cos();
//         transform.matrix[1][2] = -rad.sin();
//         transform.matrix[2][1] = rad.sin();
//         transform.matrix[2][2] = rad.cos();
//         transform.dot(self).unwrap()  
//     }

//     pub fn rotate_y(rad: f32) -> Matrix{
//         let mut transform = Matrix::zero(4,4).identity();
//         transform.matrix[0][0] = rad.cos();
//         transform.matrix[0][2] = rad.sin();
//         transform.matrix[2][0] = -rad.sin();
//         transform.matrix[2][2] = rad.cos();
//         transform  
//     }

//     pub fn rotate_z(rad: f32) -> Matrix{
//         let mut transform = Matrix::zero(4,4).identity();
//         transform.matrix[0][0] = rad.cos();
//         transform.matrix[0][1] = -rad.sin();
//         transform.matrix[1][0] = rad.sin();
//         transform.matrix[1][1] = rad.cos();
//         transform  
//     }

//     pub fn shearing(xy:f32,xz:f32,yx:f32,yz:f32,zx:f32,zy:f32) -> Matrix{
//         let mut transform = Matrix::zero(4,4).identity();
//         transform.matrix[0][1] = xy;
//         transform.matrix[0][2] = xz;
//         transform.matrix[1][0] = yx;
//         transform.matrix[1][2] = yz;
//         transform.matrix[2][0] = zx;
//         transform.matrix[2][1] = zy;
//         transform  
//     }
// }