use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Vec3<T: Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3 {
            x: $x,
            y: $y,
            z: $z,
        }
    };
}

impl<T: Add<Output = T> + Copy> Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Vec3<T> {
        vec3!(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Sub<Output = T> + Copy> Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Vec3<T> {
        vec3!(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Vec3<T> {
        vec3!(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Vec3<T> {
        vec3!(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Vec3<f32> {
    pub const ZERO: Vec3<f32> = vec3!(0.0, 0.0, 0.0);

    pub const I: Vec3<f32> = vec3!(1.0, 0.0, 0.0);

    pub const J: Vec3<f32> = vec3!(0.0, 1.0, 0.0);

    pub const K: Vec3<f32> = vec3!(0.0, 0.0, 1.0);

    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Vec3<f32> {
        self / self.magnitude()
    }
}

fn dot<T>(u: Vec3<T>, v: Vec3<T>) -> T
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    u.x * v.x + u.y * v.y + u.z * v.z
}
