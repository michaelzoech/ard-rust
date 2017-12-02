use std::ops::Mul;
use super::*;

#[derive(Copy, Clone, Debug)]
pub struct Matrix4(pub [[f64; 4]; 4]);

impl Matrix4 {

    pub fn identity() -> Matrix4 {
        Matrix4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
        Matrix4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [ x,   y,   z,  1.0],
        ])
    }

    pub fn rotation_x(rad: f64) -> Matrix4 {
        let sinr = rad.sin();
        let cosr = rad.cos();
        Matrix4([
            [1.0,   0.0,  0.0, 0.0],
            [0.0,  cosr, sinr, 0.0],
            [0.0, -sinr, cosr, 0.0],
            [0.0,   0.0,  0.0, 1.0],
        ])
    }

    pub fn rotation_y(rad: f64) -> Matrix4 {
        let sinr = rad.sin();
        let cosr = rad.cos();
        Matrix4([
            [cosr, 0.0, -sinr, 0.0],
            [ 0.0, 1.0,   0.0, 0.0],
            [sinr, 0.0,  cosr, 0.0],
            [ 0.0, 0.0,   0.0, 1.0],
        ])
    }

    pub fn rotation_z(rad: f64) -> Matrix4 {
        let sinr = rad.sin();
        let cosr = rad.cos();
        Matrix4([
            [ cosr, sinr, 0.0, 0.0],
            [-sinr, cosr, 0.0, 0.0],
            [  0.0,  0.0, 1.0, 0.0],
            [  0.0,  0.0, 0.0, 1.0],
        ])
    }

    pub fn transform_vector3(&self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self.0[0][0] * v.x + self.0[1][0] * v.y + self.0[2][0] * v.z,
            y: self.0[0][1] * v.x + self.0[1][1] * v.y + self.0[2][1] * v.z,
            z: self.0[0][2] * v.x + self.0[1][2] * v.y + self.0[2][2] * v.z,
        }
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, o: Matrix4) -> Matrix4 {
        return Matrix4([
            [
                self.0[0][0] * o.0[0][0] + self.0[1][0] * o.0[0][1] + self.0[2][0] * o.0[0][2] + self.0[3][0] * o.0[0][3],
                self.0[0][1] * o.0[0][0] + self.0[1][1] * o.0[0][1] + self.0[2][1] * o.0[0][2] + self.0[3][1] * o.0[0][3],
                self.0[0][2] * o.0[0][0] + self.0[1][2] * o.0[0][1] + self.0[2][2] * o.0[0][2] + self.0[3][2] * o.0[0][3],
                self.0[0][3] * o.0[0][0] + self.0[1][3] * o.0[0][1] + self.0[2][3] * o.0[0][2] + self.0[3][3] * o.0[0][3],
            ],
            [
                self.0[0][0] * o.0[1][0] + self.0[1][0] * o.0[1][1] + self.0[2][0] * o.0[1][2] + self.0[3][0] * o.0[1][3],
                self.0[0][1] * o.0[1][0] + self.0[1][1] * o.0[1][1] + self.0[2][1] * o.0[1][2] + self.0[3][1] * o.0[1][3],
                self.0[0][2] * o.0[1][0] + self.0[1][2] * o.0[1][1] + self.0[2][2] * o.0[1][2] + self.0[3][2] * o.0[1][3],
                self.0[0][3] * o.0[1][0] + self.0[1][3] * o.0[1][1] + self.0[2][3] * o.0[1][2] + self.0[3][3] * o.0[1][3],
            ],
            [
                self.0[0][0] * o.0[2][0] + self.0[1][0] * o.0[2][1] + self.0[2][0] * o.0[2][2] + self.0[3][0] * o.0[2][3],
                self.0[0][1] * o.0[2][0] + self.0[1][1] * o.0[2][1] + self.0[2][1] * o.0[2][2] + self.0[3][1] * o.0[2][3],
                self.0[0][2] * o.0[2][0] + self.0[1][2] * o.0[2][1] + self.0[2][2] * o.0[2][2] + self.0[3][2] * o.0[2][3],
                self.0[0][3] * o.0[2][0] + self.0[1][3] * o.0[2][1] + self.0[2][3] * o.0[2][2] + self.0[3][3] * o.0[2][3],
            ],
            [
                self.0[0][0] * o.0[3][0] + self.0[1][0] * o.0[3][1] + self.0[2][0] * o.0[3][2] + self.0[3][0] * o.0[3][3],
                self.0[0][1] * o.0[3][0] + self.0[1][1] * o.0[3][1] + self.0[2][1] * o.0[3][2] + self.0[3][1] * o.0[3][3],
                self.0[0][2] * o.0[3][0] + self.0[1][2] * o.0[3][1] + self.0[2][2] * o.0[3][2] + self.0[3][2] * o.0[3][3],
                self.0[0][3] * o.0[3][0] + self.0[1][3] * o.0[3][1] + self.0[2][3] * o.0[3][2] + self.0[3][3] * o.0[3][3],
            ],
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::f64::consts::PI;

    #[test]
    fn right_most_matrix_applies_first() {
        let m = Matrix4::rotation_y(PI*0.5) * Matrix4::rotation_x(PI*0.5);
        let u = m.transform_vector3(Vector3 { x: 0.0, y: 1.0, z: 0.0 });
        assert_close!(Vector3 { x: 1.0, y: 0.0, z: 0.0 }, u);
    }

    #[test]
    fn positive_rotation_is_counterclockwise() {
        let mx = Matrix4::rotation_x(PI*0.5);
        let ux = mx.transform_vector3(Vector3 { x: 0.0, y: 0.0, z: 1.0 });
        assert_close!(Vector3 { x: 0.0, y: -1.0, z: 0.0 }, ux);
        let my = Matrix4::rotation_y(PI*0.5);
        let uy = my.transform_vector3(Vector3 { x: 0.0, y: 0.0, z: 1.0 });
        assert_close!(Vector3 { x: 1.0, y: 0.0, z: 0.0 }, uy);
        let mz = Matrix4::rotation_z(PI*0.5);
        let uz = mz.transform_vector3(Vector3 { x: 1.0, y: 0.0, z: 0.0 });
        assert_close!(Vector3 { x: 0.0, y: 1.0, z: 0.0 }, uz);
    }

    #[test]
    fn transform_vector_does_not_translate() {
        let m = Matrix4::translation(1.0, 1.0, 1.0);
        let u = m.transform_vector3(Vector3 { x: 1.0, y: 1.0, z: 1.0 });
        assert_close!(Vector3 { x: 1.0, y: 1.0, z: 1.0 }, u);
    }
}
