
use std::fmt;
use std::f32;
use std::ops::{Add, Sub, Mul};
use std::cmp::Ordering;



#[derive(Debug, Copy, Clone, PartialOrd)]
pub struct Cvector {
    pub x: f32,
    pub y: f32,
    pub z: f32
}


impl Cvector {

    pub fn from_parts(x: f32, y: f32, z: f32) -> Cvector {
        Cvector{x:x, y:y, z:z}
    }

    pub fn from_array(arr: &[f32; 3]) -> Cvector {
        Cvector{x:arr[0], y:arr[1], z:arr[2]}
    }

    pub fn from_vec(vec: &Vec<f32>) -> Cvector {
        assert_eq!(vec.len(), 3);
        Cvector{x:vec[0], y:vec[1], z:vec[2]}
    }


    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn distance(&self, other: &Cvector) -> f32 {
        let delta_x = self.x - other.x;
        let delta_y = self.y - other.y;
        let delta_z = self.z - other.z;
        let temp = Cvector{x:delta_x, y:delta_y, z:delta_z};

        temp.length()
    }

    pub fn cross(&self, other: &Cvector) -> Cvector {
        Cvector{x: self.y*other.z - self.z*other.y,
                y: self.z*other.x - self.x*other.z,
                z: self.x*other.y - self.y*other.x}
    }

    pub fn dot(&self, other: &Cvector) -> f32 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn eps_cmp(&self, other: &Cvector) -> bool {
        ((self.x - other.x).abs() < 2.0*f32::EPSILON &&
         (self.y - other.y).abs() < 2.0*f32::EPSILON &&
         (self.z - other.z).abs() < 2.0*f32::EPSILON)
    }
}


impl Sub for Cvector {
    type Output = Cvector;

    fn sub(self, other: Cvector) -> Cvector {
        Cvector{x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z}
    }
}


impl Add for Cvector {
    type Output = Cvector;

    fn add(self, other: Cvector) -> Cvector {
        Cvector{x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z}
    }
}


impl Mul<f32> for Cvector {
    type Output = Cvector;

    fn mul(self, rhs: f32) -> Cvector {
        Cvector::from_parts(self.x*rhs, self.y*rhs, self.z*rhs)
    }
}


impl Ord for Cvector {
    fn cmp(&self, other: &Cvector) -> Ordering {
        if self.x != other.x {
            self.x.partial_cmp(&other.x).unwrap()
        } else if self.y != other.y {
            self.y.partial_cmp(&other.y).unwrap()
        } else {
            self.z.partial_cmp(&other.z).unwrap()
        }
    }
}


impl PartialEq for Cvector {
    fn eq(&self, other: &Cvector) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl Eq for Cvector {}

impl fmt::Display for Cvector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_from_parts() {
        let origin = Cvector::from_parts(0.0, 0.0, 0.0);
        assert_eq!(origin.x, 0.0);
        assert_eq!(origin.y, 0.0);
        assert_eq!(origin.z, 0.0);

        let another = Cvector::from_parts(1.0, 2.0, 3.0);
        assert_eq!(another.x, 1.0);
        assert_eq!(another.y, 2.0);
        assert_eq!(another.z, 3.0);
    }

    #[test]
    fn create_from_array() {
        let arr: [f32; 3] = [0.0; 3];
        let origin = Cvector::from_array(&arr);
        assert_eq!(origin.x, 0.0);
        assert_eq!(origin.y, 0.0);
        assert_eq!(origin.z, 0.0);

        let arr: [f32; 3] = [1.0, 2.0, 3.0];
        let another = Cvector::from_array(&arr);
        assert_eq!(another.x, 1.0);
        assert_eq!(another.y, 2.0);
        assert_eq!(another.z, 3.0);
    }

    #[test]
    fn create_from_vec() {
        let vec = vec![0.0; 3];
        let origin = Cvector::from_vec(&vec);
        assert_eq!(origin.x, 0.0);
        assert_eq!(origin.y, 0.0);
        assert_eq!(origin.z, 0.0);

        let vec = vec![1.0, 2.0, 3.0];
        let another = Cvector::from_vec(&vec);
        assert_eq!(another.x, 1.0);
        assert_eq!(another.y, 2.0);
        assert_eq!(another.z, 3.0);
    }

    #[test]
    fn length() {
        let origin = Cvector::from_parts(0.0, 0.0, 0.0);
        let len = origin.length();
        assert_eq!(len, 0.0);

        let unit = Cvector::from_parts(1.0, 0.0, 0.0);
        let len = unit.length();
        assert_eq!(len, 1.0);

        let unit = Cvector::from_parts(0.0, 1.0, 0.0);
        let len = unit.length();
        assert_eq!(len, 1.0);

        let unit = Cvector::from_parts(0.0, 0.0, 1.0);
        let len = unit.length();
        assert_eq!(len, 1.0);

        let unit = Cvector::from_parts(0.5, 0.5, 0.7071067811865476);
        let len = unit.length();
        assert_eq!(len, 1.0);

        let unit = Cvector::from_parts(-0.5, 0.5, -0.7071067811865476);
        let len = unit.length();
        assert_eq!(len, 1.0);

        let another = Cvector::from_parts(27.0, 44.0,  96.0);
        let len = another.length();
        assert_eq!(len, 109.0);
    }

    #[test]
    fn subtraction() {
        assert_eq!(Cvector::from_parts(1.0, 2.0, 3.0) - Cvector::from_parts(0.0, 0.0, 0.0),
                   Cvector::from_parts(1.0, 2.0, 3.0));
        assert_eq!(Cvector::from_parts(0.0, 0.0, 0.0) - Cvector::from_parts(1.0, 2.0, 3.0),
                   Cvector::from_parts(-1.0, -2.0, -3.0));
        assert_eq!(Cvector::from_parts(1.0, 2.0, 3.0) - Cvector::from_parts(1.0, 2.0, 3.0),
                   Cvector::from_parts(0.0, 0.0, 0.0));
        assert_eq!(Cvector::from_parts(-1.0, -2.0, -3.0) - Cvector::from_parts(0.0, 0.0, 0.0),
                   Cvector::from_parts(-1.0, -2.0, -3.0));
        assert_eq!(Cvector::from_parts(-1.0, -2.0, -3.0) - Cvector::from_parts(-1.0, -2.0, -3.0),
                   Cvector::from_parts(0.0, 0.0, 0.0));
    }

    #[test]
    fn cross() {
        {
            let a = Cvector::from_parts(1.0, 2.0, 3.0);
            let b = Cvector::from_parts(4.0, 5.0, 6.0);
            assert_eq!(a.cross(&b), Cvector::from_parts(-3.0, 6.0, -3.0));
            assert_eq!(b.cross(&a), Cvector::from_parts(3.0, -6.0, 3.0));
        }
        {
            let a = Cvector::from_parts(77.0, 12.0, 23.0);
            let b = Cvector::from_parts(44.0, 15.0, -6.0);
            assert_eq!(a.cross(&b), Cvector::from_parts(-417.0, 1474.0, 627.0));
            assert_eq!(b.cross(&a), Cvector::from_parts(417.0, -1474.0, -627.0));
        }
    }
}
