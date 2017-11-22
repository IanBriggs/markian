

use std::fmt;

extern crate cvector;

use cvector::{Cvector};



#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Triangle {
    pub unit_normal: Cvector,
    pub v1: Cvector,
    pub v2: Cvector,
    pub v3: Cvector,
}


impl Triangle {

    pub fn from_cvectors(unit_normal: &Cvector, v1: &Cvector, v2: &Cvector, v3: &Cvector) -> Triangle {
        Triangle{unit_normal:unit_normal.clone(), v1:v1.clone(), v2:v2.clone(), v3:v3.clone()}
    }

    pub fn from_verts(v1: &Cvector, v2: &Cvector, v3: &Cvector) -> Triangle {
        let origin = Cvector::from_parts(0.0, 0.0, 0.0);
        let temp = Triangle{unit_normal:origin, v1:v1.clone(), v2:v2.clone(), v3:v3.clone()};
        let unit_normal = temp.recalc_unit_normal();

        Triangle{unit_normal:unit_normal, v1:v1.clone(), v2:v2.clone(), v3:v3.clone()}
    }


    pub fn recalc_unit_normal(&self) -> Cvector {
        let n1 = (self.v1 - self.v2).cross(&(self.v3 - self.v2));
        let n2 = (self.v2 - self.v3).cross(&(self.v1 - self.v3));
        let n3 = (self.v3 - self.v1).cross(&(self.v2 - self.v1));
        let nx = (n1.x + n2.x + n3.x) / 3.0;
        let ny = (n1.y + n2.y + n3.y) / 3.0;
        let nz = (n1.z + n2.z + n3.z) / 3.0;
        let n = Cvector::from_parts(nx, ny, nz);
        let l = n.length();

        Cvector::from_parts(-nx/l, -ny/l, -nz/l)
    }

}


impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{1}, {2}, {3}, {0}]", self.unit_normal, self.v1, self.v2, self.v3)
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
