
use std::f32;
use std::fmt;

extern crate cvector;
use cvector::Cvector;
extern crate triangle;
use triangle::Triangle;

pub struct Ray {
    origin: Cvector,
    vector: Cvector
}
const RAY_EPSILON: f32 = 1.0e-6;

impl Ray {

    pub fn from_parts(origin:&Cvector, vector:&Cvector) -> Ray {
        Ray{origin:origin.clone(), vector:vector.clone()}
    }

    pub fn from_tri(tri: &Triangle) -> Ray {
        let vector = tri.unit_normal.clone() * -1.0;
        let center = Cvector::from_parts((tri.v1.x + tri.v2.x + tri.v3.x) / 3.0,
                                         (tri.v1.y + tri.v2.y + tri.v3.y) / 3.0,
                                         (tri.v1.z + tri.v2.z + tri.v3.z) / 3.0);
        Ray{origin:center - vector*(4.0*RAY_EPSILON), vector:vector}
    }

    // From: https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    pub fn intersect(&self, tri: &Triangle) -> Option<Cvector> {
        let v1 = tri.v1;
        let v2 = tri.v2;
        let v3 = tri.v3;

        let edge1 = v2 - v1;
        let edge2 = v3 - v1;
        let h = self.vector.cross(&edge2);
        let a = edge1.dot(&h);
        if a > -RAY_EPSILON && a < RAY_EPSILON {
            return None;
        }

        let f = a.recip();
        let s = self.origin - v1;
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * self.vector.dot(&q);
        if v < 0.0 || u+v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);
        if t > RAY_EPSILON {
            let intersection = self.origin + self.vector * t;
            return Some(intersection);
        } else {
            return None;
        }
    }


    fn internal_all_intersections(&self, tris: &Vec<Triangle>) -> Vec<Cvector> {
        let mut raw: Vec<Cvector> = tris.iter().filter_map(|t| self.intersect(&t)).collect();
        raw.sort();
        raw
    }

    fn internal_check_intersections(&self, inters: &Vec<Cvector>) -> bool {
        for i in 1..inters.len() {
            if inters[i-1] == inters[i] {
                return false
            }
        }

        true
    }

    pub fn all_intersections(&self, tris: &Vec<Triangle>) -> Vec<Cvector> {
        let raw = self.internal_all_intersections(tris);
        if self.internal_check_intersections(&raw) {
            return raw
        }

        let px = Cvector::from_parts(self.vector.x, -self.vector.z, self.vector.y) * (2.0*RAY_EPSILON);
        let perturbed_x = Ray::from_parts(&(self.origin + px), &self.vector);
        let raw_x = perturbed_x.internal_all_intersections(tris);
        if self.internal_check_intersections(&raw_x) {
            return raw_x
        }

        let py = Cvector::from_parts(self.vector.z, self.vector.y, -self.vector.x) * (2.0*RAY_EPSILON);
        let perturbed_y = Ray::from_parts(&(self.origin + py), &self.vector);
        let raw_y = perturbed_y.internal_all_intersections(tris);
        if self.internal_check_intersections(&raw_y) {
            return raw_y
        }

        let pz = Cvector::from_parts(-self.vector.y, self.vector.x, self.vector.z) * (2.0*RAY_EPSILON);
        let perturbed_z = Ray::from_parts(&(self.origin + pz), &self.vector);
        let raw_z = perturbed_z.internal_all_intersections(tris);
        if self.internal_check_intersections(&raw_z) {
            return raw_z
        }

        unreachable!();
    }

}


impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.origin, self.vector)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
