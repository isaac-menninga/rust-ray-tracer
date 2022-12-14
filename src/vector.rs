use std::ops::*;

#[derive(Clone, Copy, Debug)]
pub struct Vector(pub f64, pub f64, pub f64);

impl Vector {
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn dot(&self, other: Vector) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Vector) -> Vector {
        Vector(
            self.1 * other.2 - self.2 * other.1,
            -(self.0 * other.2 - self.2 * other.0),
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn squared_length(self) -> f64 {
        self.dot(self)
    }
    pub fn length(self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn reflect(self, n: Vector) -> Vector {
        self - 2.0 * self.dot(n) * n
    }

    pub fn to_u8(&self) -> [u8; 3] {
        fn u(f: f64) -> u8 {
            if f < 0.0 {
                0
            } else if f >= 1.0 {
                255
            } else {
                (f * 255.9) as i32 as u8
            }
        }
        [u(self.0), u(self.1), u(self.2)]
    }

    pub fn to_rgb(&self) -> lodepng::RGB<u8> {
        let v = self.to_u8();

        lodepng::RGB {
            r: v[0],
            g: v[1],
            b: v[2],
        }
    }

    pub fn to_unit_vector(&self) -> Vector {
        *self / self.length()
    }

    pub fn print(&self) {
        println!("{} {} {}", self.x(), self.y(), self.x());
    }

    pub fn near_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;
        self.0.abs() < EPS && self.1.abs() < EPS && self.2.abs() < EPS
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector(-self.0, -self.1, -self.2)
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, v: Vector) -> Vector {
        Vector(self * v.0, self * v.1, self * v.2)
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;
    fn mul(self, v: Vector) -> Vector {
        Vector(self.0 * v.0, self.1 * v.1, self.2 * v.2)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, r: f64) -> Vector {
        (1.0 / r) * self
    }
}
