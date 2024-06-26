use crate::random;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub const fn black() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub const fn white() -> Self {
        Self(1.0, 1.0, 1.0)
    }

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    pub fn cross(&self, other: &Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
    pub fn unit_vec(&self) -> Self {
        *self / self.length()
    }

    pub fn random_unit() -> Self {
        Self::random(0.0, 1.0)
    }
    pub fn random(lower: f64, upper: f64) -> Self {
        Self(
            random::range(lower, upper),
            random::range(lower, upper),
            random::range(lower, upper),
        )
    }
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let point = Self::random(-1.0, 1.0);
            if point.length_squared() < 1.0 {
                return point;
            }
        }
    }
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vec()
    }
    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let point = Self::random_in_unit_sphere();
        if point.dot(normal) > 0.0 {
            point
        } else {
            -point
        }
    }
    pub fn random_in_unit_disk() -> Self {
        loop {
            let point = Self(random::range(-1.0, 1.0), random::range(-1.0, 1.0), 0.0);
            if point.length_squared() < 1.0 {
                return point;
            }
        }
    }

    pub fn reflect(&self, other: &Self) -> Self {
        *self - 2.0 * self.dot(other) * *other
    }
    pub fn refract(&self, normal: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *normal;
        r_out_perp + r_out_parallel
    }

    pub fn near_zero(&self) -> bool {
        let near_zero = |f: f64| f.abs() < 1e-8;
        near_zero(self.0) && near_zero(self.1) && near_zero(self.2)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}
impl Add for Vec3 {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}
impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
        self.1 += rhs;
        self.2 += rhs;
    }
}
impl Add<Vec3> for f64 {
    type Output = Vec3;
    fn add(self, mut rhs: Vec3) -> Self::Output {
        rhs += self;
        rhs
    }
}
impl Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(mut self, rhs: f64) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}
impl Div for Vec3 {
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

pub type Color = Vec3;
pub type Point = Vec3;
