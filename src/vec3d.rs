use std::ops::{Add, Sub, Mul, MulAssign, AddAssign, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3d<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3d { x, y, z }
    }
}

impl<T> Add for Vec3d<T> where T: Add<Output=T> {
    type Output = Vec3d<T>;

    fn add(self, rhs: Vec3d<T>) -> Self::Output {
        Vec3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Sub for Vec3d<T> where T: Sub<Output=T> {
    type Output = Vec3d<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> AddAssign for Vec3d<T> where T : AddAssign {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> SubAssign for Vec3d<T> where T : SubAssign {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T, U> Mul<U> for Vec3d<T> where T : Mul<U, Output=T>, U : Copy {
    type Output = Vec3d<T>;

    fn mul(self, rhs: U) -> Self::Output {
        Vec3d::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T> Mul<Vec3d<T>> for f64 where T : Mul<f64, Output=T> {
    type Output = Vec3d<T>;

    fn mul(self, rhs: Vec3d<T>) -> Self::Output {
        rhs * self
    }
}

impl<T, U> MulAssign<U> for Vec3d<T> where T : MulAssign<U>, U : Copy {
    fn mul_assign(&mut self, rhs: U) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3d::Vec3d;

    #[test]
    fn test_add() {
        let v1 = Vec3d::new(1.0, 2.0, 3.0);
        let v2 = Vec3d::new(4.0, 5.0, 6.0);
        let res = Vec3d::new(5.0, 7.0, 9.0);
        assert_eq!(v1 + v2, res);
    }

    #[test]
    fn test_add_int() {
        let v1 = Vec3d::new(1, 2, 3);
        let v2 = Vec3d::new(4, 5, 6);
        let res = Vec3d::new(5, 7, 9);
        assert_eq!(v1 + v2, res);
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vec3d::new(1.0, 2.0, 3.0);
        let v2 = Vec3d::new(4.0, 5.0, 6.0);
        let res = Vec3d::new(5.0, 7.0, 9.0);
        v1 += v2;
        assert_eq!(v1, res);
    }

    #[test]
    fn test_add_assign_int() {
        let mut v1 = Vec3d::new(1, 2, 3);
        let v2 = Vec3d::new(4, 5, 6);
        let res = Vec3d::new(5, 7, 9);
        v1 += v2;
        assert_eq!(v1, res);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3d::new(1.0, 2.0, 3.0);
        let v2 = Vec3d::new(4.0, 5.0, 6.0);
        let res = Vec3d::new(3.0, 3.0, 3.0);
        assert_eq!(v2 - v1, res);
    }


    #[test]
    fn test_sub_assign() {
        let v1 = Vec3d::new(1.0, 2.0, 3.0);
        let mut v2 = Vec3d::new(4.0, 5.0, 6.0);
        let res = Vec3d::new(3.0, 3.0, 3.0);
        v2 -= v1;
        assert_eq!(v2, res);
    }


    #[test]
    fn test_mul() {
        let v = Vec3d::new(4.0, 5.0, 6.0);
        let res = Vec3d::new(12.0, 15.0, 18.0);
        assert_eq!(v * 3.0, res);
        assert_eq!(3.0 * v, res);
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vec3d::new(4.0, 5.0, 6.0);
        v *= 3.0;
        let res = Vec3d::new(12.0, 15.0, 18.0);
        assert_eq!(v, res);
    }

    #[test]
    fn test_mul_assign_int() {
        let mut v = Vec3d::new(4, 5, 6);
        v *= -1;
        let res = Vec3d::new(-4, -5, -6);
        assert_eq!(v, res);
    }
}