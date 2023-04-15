use rand::random;
use std::ops::{Add, Div, Mul, Neg, Sub};

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

impl<U: Copy, T: Add<Output = U> + Copy> Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<U>;

    fn add(self, rhs: Vec3<T>) -> Vec3<U> {
        vec3!(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<U: Copy, T: Sub<Output = U> + Copy> Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<U>;

    fn sub(self, rhs: Vec3<T>) -> Vec3<U> {
        vec3!(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<U: Copy, T: Mul<Output = U> + Copy> Mul<T> for Vec3<T> {
    type Output = Vec3<U>;

    fn mul(self, rhs: T) -> Vec3<U> {
        vec3!(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3<f32>> for f32 {
    type Output = Vec3<f32>;

    fn mul(self, rhs: Vec3<f32>) -> Vec3<f32> {
        rhs * self
    }
}

impl<U: Copy, T: Div<Output = U> + Copy> Div<T> for Vec3<T> {
    type Output = Vec3<U>;

    fn div(self, rhs: T) -> Vec3<U> {
        vec3!(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<U: Copy, T: Neg<Output = U> + Copy> Neg for Vec3<T> {
    type Output = Vec3<U>;

    fn neg(self) -> Vec3<U> {
        vec3!(-self.x, -self.y, -self.z)
    }
}

impl<T: std::fmt::Display + Copy> std::fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[allow(dead_code)]
impl Vec3<f32> {
    pub const ZERO: Vec3<f32> = vec3!(0.0, 0.0, 0.0);

    pub const I: Vec3<f32> = vec3!(1.0, 0.0, 0.0);

    pub const J: Vec3<f32> = vec3!(0.0, 1.0, 0.0);

    pub const K: Vec3<f32> = vec3!(0.0, 0.0, 1.0);

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3<f32> {
        *self / self.magnitude()
    }

    pub fn random_unit_ball() -> Vec3<f32> {
        loop {
            let v = Vec3 {
                x: 2.0 * random::<f32>() - 1.0,
                y: 2.0 * random::<f32>() - 1.0,
                z: 2.0 * random::<f32>() - 1.0,
            };
            if dot(v, v) < 1.0 {
                return v;
            }
        }
    }

    pub fn random_unit() -> Vec3<f32> {
        Self::random_unit_ball().normalize()
    }
}

pub fn dot<T>(u: Vec3<T>, v: Vec3<T>) -> T
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross<T>(u: Vec3<T>, v: Vec3<T>) -> Vec3<T>
where
    T: Sub<Output = T> + Mul<Output = T> + Copy,
{
    vec3!(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x
    )
}
