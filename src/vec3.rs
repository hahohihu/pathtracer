use std::{
    fmt::Display,
    ops::{Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

macro_rules! create_vec3 {
    ($name:ident) => {
        #[derive(Default, Debug, Clone, Copy)]
        pub struct $name(f64, f64, f64);

        impl $name {
            pub fn new(x: f64, y: f64, z: f64) -> Self {
                $name(x, y, z)
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
                self.0 * other.0 + // rustfmt guard
                self.1 * other.1 + // rustfmt guard
                self.2 * other.2
            }

            pub fn cross(&self, other: &Self) -> Self {
                Self(
                    self.1 * self.2 - self.2 * self.1,
                    self.2 * self.0 - self.0 * self.2,
                    self.0 * self.1 - self.1 * self.0,
                )
            }

            pub fn unit_vec(&self) -> Self {
                *self / self.length()
            }
        }

        // vector operations: vec * vec
        impl SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
                self.1 -= rhs.1;
                self.2 -= rhs.2;
            }
        }

        impl Sub for $name {
            type Output = Self;
            fn sub(mut self, rhs: Self) -> Self::Output {
                self -= rhs;
                self
            }
        }

        impl Neg for $name {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self(-self.0, -self.1, -self.2)
            }
        }

        impl MulAssign for $name {
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0;
                self.1 *= rhs.1;
                self.2 *= rhs.2;
            }
        }

        impl Mul<$name> for $name {
            type Output = Self;
            fn mul(mut self, rhs: Self) -> Self::Output {
                self *= rhs;
                self
            }
        }

        impl DivAssign for $name {
            fn div_assign(&mut self, rhs: Self) {
                self.0 /= rhs.0;
                self.1 /= rhs.1;
                self.2 /= rhs.2;
            }
        }

        impl Div for $name {
            type Output = Self;
            fn div(mut self, rhs: Self) -> Self::Output {
                self /= rhs;
                self
            }
        }

        // scalar ops: scalar * vec

        impl Mul<f64> for $name {
            type Output = Self;
            fn mul(self, rhs: f64) -> Self::Output {
                Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
            }
        }

        impl Div<f64> for $name {
            type Output = Self;
            fn div(self, rhs: f64) -> Self::Output {
                Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
            }
        }
    };
}

create_vec3!(Point3);

impl Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

create_vec3!(Color);

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn rgb_to_int(value: f64) -> i64 {
            (value * 255.999) as i64
        }
        write!(f, "{} {} {}", rgb_to_int(self.0), rgb_to_int(self.1), rgb_to_int(self.2))
    }
}

