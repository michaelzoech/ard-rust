use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;
use super::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f64) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vector2> for f64 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}


impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Vector2 {

    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 {
            x: x,
            y: y,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3 {

    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Vector3 {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vector3 {

    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign<f64> for Vector3 {

    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector3 {

    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl CloseEq for Vector3 {

    fn close_eq(&self, rhs: &Vector3) -> bool {
        self.x.close_eq(&rhs.x) &&
        self.y.close_eq(&rhs.y) &&
        self.z.close_eq(&rhs.z)
    }
}

impl Vector3 {

    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&mut self) {
        let one_over_len = 1.0 / self.length();
        self.x *= one_over_len;
        self.y *= one_over_len;
        self.z *= one_over_len;
    }

    pub fn normalized(&self) -> Vector3 {
        let one_over_len = 1.0 / self.length();
        Vector3::new(self.x * one_over_len, self.y * one_over_len, self.z * one_over_len)
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn reflect(&self, reflector: &Vector3) -> Vector3 {
        *self - 2.0 * self.dot(reflector) * *reflector
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_traits() {
        let u = Vector3::new(1.0, 2.0, 3.0);
        let v = Vector3::new(2.0, 3.0, 4.0);
        let result = Vector3::new(3.0, 5.0, 7.0);
        assert_close!(result, u + v);
        let mut actual = u;
        actual += v;
        assert_close!(result, actual);
    }

    #[test]
    fn sub_traits() {
        let u = Vector3::new(1.0, 2.0, 3.0);
        let v = Vector3::new(0.5, 1.0, 1.5);
        let result = Vector3::new(0.5, 1.0, 1.5);
        assert_close!(result, u - v);
        let mut actual = u;
        actual -= v;
        assert_close!(result, actual);
    }

    #[test]
    fn length_tests() {
        assert_close!(1.0, Vector3::new(1.0, 0.0, 0.0).length());
    }

    #[test]
    fn normalize_self() {
        let mut v = Vector3::new(1.0, 2.0, 3.0);
        v.normalize();
        assert_close!(1.0, v.length());
    }

    #[test]
    fn normalized_returns_unit_length_vector() {
        let v = Vector3::new(1.0, 1.0, 1.0);
        let vn = v.normalized();
        assert_close!(1.0, vn.length());
    }

    #[test]
    fn normalized_point_in_same_direction() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let len = v.length();
        let vn = v.normalized();
        assert_close!(v, vn * len);
    }

    #[test]
    fn cross_adherse_to_right_hand_side_rul() {
        let u = Vector3::new(1.0, 0.0, 0.0);
        let v = Vector3::new(0.0, 1.0, 0.0);
        assert_close!(Vector3::new(0.0, 0.0, 1.0), u.cross(&v));
    }
}