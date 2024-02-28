use std::ops::{self};

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(non_camel_case_types)]
pub type point3 = vec3;
#[allow(non_camel_case_types)]
pub type color = vec3;

impl vec3 {
    // generic functions
    pub fn new(x: f64, y: f64, z: f64) -> vec3 {
        vec3 { x, y, z }
    }

    pub fn from_scalar(f: f64) -> vec3 {
        vec3 { x: f, y: f, z: f }
    }

    pub fn zero() -> vec3 {
        vec3::from_scalar(0.0)
    }

    pub fn identity() -> vec3 {
        vec3::from_scalar(1.0)
    }

    // instance based functions

    pub fn x(self) -> f64 {
        self.x
    }

    pub fn y(self) -> f64 {
        self.y
    }

    pub fn z(self) -> f64 {
        self.z
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalized(self) -> vec3 {
        let len2 = self.length_squared();
        // if length is close enough to 1
        if f64::abs(len2 - 1.0) <= f64::EPSILON {
            self
        } else {
            let len = len2.sqrt();
            vec3 {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        }
    }

    pub fn normalize(mut self) {
        let len2 = self.length_squared();
        if f64::abs(len2 - 1.0) >= f64::EPSILON {
            let len = len2.sqrt();
            self.x /= len;
            self.y /=len;
            self.z /=len;
        }
    }

    pub fn dot(u: vec3, v:vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: vec3, v: vec3) -> vec3 {
        vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.z,
        }
    }
}


// for printing
impl std::fmt::Display for vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.2}, {:.2}, {:.2}]", self.x, self.y, self.z)
    }
}

// overloading +
impl ops::Add for vec3 {
    type Output = vec3;
    fn add(self, rhs: vec3) -> Self::Output {
        vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// overloading +=
impl ops::AddAssign for vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// overloading -
impl ops::Sub for vec3 {
    type Output = vec3;

    fn sub(self, rhs: vec3) -> Self::Output {
        vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// overlaoding -=
impl ops::SubAssign for vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// overloading * scalar
impl ops::Mul<f64> for vec3 {
    type Output = vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// overloading * vec3
impl ops::Mul for vec3 {
    type Output = vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

// overloading *= scalar
impl ops::MulAssign<f64> for vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

// overloading *= vec3
impl ops::MulAssign for vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

// overloading / scalar
impl ops::Div<f64> for vec3 {
    type Output = vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * rhs.recip()
    }
}

// overloading /= scalar
impl ops::DivAssign<f64> for vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

// overloading -v
impl ops::Neg for vec3 {
    type Output = vec3;

    fn neg(self) -> Self::Output {
        vec3 {
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg(),
        }
    }
}


#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct ray {
    origin: point3,
    direction: vec3
}

impl ray {
    pub fn new(origin: vec3, direction: vec3) -> ray {
        ray {
            origin,
            direction
        }
    }

    pub fn at(self, t: f64) -> point3 {
        self.origin + self.direction * t
    }

    pub fn direction(self) -> vec3 {
        self.direction
    }

    pub fn origin(self) -> point3 {
        self.origin
    }
}