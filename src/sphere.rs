use crate::vector::Vector;
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct Hit {
    pub t: f32,
    pub p: Vector,
    pub normal: Vector,
    // pub material: &'obj Material
}

#[derive(Clone, Copy)]
pub struct Sphere {
    center: Vector,
    radius: f32
}

// Fix floating point bug
const T_PRECISION: f32 = 0.00001;

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, r: f32) -> Self {
        Self {
            center: Vector(x, y, z),
            radius: r
        }
    }

    pub fn ray_intersect(&self, r: Ray) -> Option<Hit> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let hb = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = hb * hb - a * c;

        if discriminant > 0.0 {
            let t1 = (-hb + discriminant.sqrt()) / 2.0;
            let t2 = (-hb - discriminant.sqrt()) / 2.0;

            if (t1 > 0.0) & (t2 > 0.0) {
                let mut t = t1;
                if t1 > t2 {
                    t = t2;
                }

                if t >= T_PRECISION {
                    let intersection = r.line_to_p(t);

                    return Some(Hit {
                        t: t,
                        p: intersection,
                        normal: (intersection - self.center) / self.radius,
                        // material: &*self.material
                    })
                }
            }
        }

        return None;
    }
}
