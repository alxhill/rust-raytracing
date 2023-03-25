use std::ops::{Index, IndexMut, Mul};

#[derive(Copy, Clone)]
pub struct Matrix {
    matrix: [[f64; 4]; 4],
}

impl Matrix {
    pub fn new() -> Self {
        Matrix {
            matrix: [[0.0; 4]; 4],
        }
    }

    pub fn identity() -> Self {
        let mut m = Self::new();
        m.matrix[0][0] = 1.0;
        m.matrix[1][1] = 1.0;
        m.matrix[2][2] = 1.0;
        m.matrix[3][3] = 1.0;
        m
    }

    pub fn translate(x: f64, y: f64, z: f64) -> Self {
        let mut m = Self::identity();
        m.matrix[0][3] = x;
        m.matrix[1][3] = y;
        m.matrix[2][3] = z;
        m
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Self {
        let mut m = Self::identity();
        m.matrix[0][0] = x;
        m.matrix[1][1] = y;
        m.matrix[2][2] = z;
        m
    }

    pub fn rotate_x(angle: f64) -> Self {
        let mut m = Self::identity();
        let c = angle.cos();
        let s = angle.sin();
        m.matrix[1][1] = c;
        m.matrix[1][2] = -s;
        m.matrix[2][1] = s;
        m.matrix[2][2] = c;
        m
    }

    pub fn rotate_y(angle: f64) -> Self {
        let mut m = Self::identity();
        let c = angle.cos();
        let s = angle.sin();
        m.matrix[0][0] = c;
        m.matrix[0][2] = s;
        m.matrix[2][0] = -s;
        m.matrix[2][2] = c;
        m
    }

    pub fn rotate_z(angle: f64) -> Self {
        let mut m = Self::identity();
        let c = angle.cos();
        let s = angle.sin();
        m.matrix[0][0] = c;
        m.matrix[0][1] = -s;
        m.matrix[1][0] = s;
        m.matrix[1][1] = c;
        m
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.matrix[row][col]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.matrix[row][col]
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self[(i, k)] * rhs[(k, j)];
                }
                result[(i, j)] = sum;
            }
        }
        result
    }
}
