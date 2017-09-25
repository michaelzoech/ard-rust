use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Ray3 {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl CloseEq for Ray3 {

    fn close_eq(&self, rhs: &Ray3) -> bool {
        self.origin.close_eq(&rhs.origin) && self.direction.close_eq(&rhs.direction)
    }
}

impl Ray3 {

}

#[cfg(test)]
mod tests {
    use super::*;

}