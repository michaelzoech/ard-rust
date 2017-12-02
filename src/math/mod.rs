mod matrix;
mod ray;
mod vector;

pub use self::matrix::*;
pub use self::ray::*;
pub use self::vector::*;

pub trait CloseEq<RHS=Self> {

    fn close_eq(&self, rhs: &RHS) -> bool;

    fn close_ne(&self, rhs: &RHS) -> bool {
        !self.close_eq(rhs)
    }
}

impl CloseEq for f64 {
    fn close_eq(&self, rhs: &f64) -> bool {
        let diff = (rhs - self).abs();
        diff < 0.0000001
    }
}
