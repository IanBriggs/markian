
extern crate cvector;
use cvector::Cvector;
extern crate triangle;
use triangle::Triangle;
extern crate ray;
use ray::Ray;

extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt};

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


pub fn read_binary_stl(filename: &str) -> () {
    println!("Filename: '{}'", filename);

    let file = File::open(filename).unwrap();
    let mut buf_reader = BufReader::new(file);

    let mut header_arr: [u8; 80] = [0; 80];
    buf_reader.read_exact(&mut header_arr).unwrap();
    let header = String::from_utf8(header_arr.to_vec()).unwrap();
    println!("Header: '{}'", header);

    let tri_count = buf_reader.read_u32::<LittleEndian>().unwrap();
    // println!("  (tri-count {})", tri_count);

    // println!("  (triangles");
    let mut triangles = Vec::new();
    let mut bad_normals = 0;
    for _ in 1..(tri_count + 1) {

        let mut tri_normal = vec![0.0; 3];
        let mut tri_vertex_1 = vec![0.0; 3];
        let mut tri_vertex_2 = vec![0.0; 3];
        let mut tri_vertex_3 = vec![0.0; 3];
        unsafe {
            buf_reader.read_f32_into_unchecked::<LittleEndian>(&mut tri_normal).unwrap();
            buf_reader.read_f32_into_unchecked::<LittleEndian>(&mut tri_vertex_1).unwrap();
            buf_reader.read_f32_into_unchecked::<LittleEndian>(&mut tri_vertex_2).unwrap();
            buf_reader.read_f32_into_unchecked::<LittleEndian>(&mut tri_vertex_3).unwrap();
        }
        let unit_normal = Cvector::from_vec(&tri_normal);
        let v1 = Cvector::from_vec(&tri_vertex_1);
        let v2 = Cvector::from_vec(&tri_vertex_2);
        let v3 = Cvector::from_vec(&tri_vertex_3);

        buf_reader.read_u16::<LittleEndian>().unwrap();

        let triangle = Triangle::from_verts(&v1, &v2, &v3);
        if triangle.unit_normal.eps_cmp(&unit_normal) == false {
            bad_normals += 1;
        }
        triangles.push(triangle);

        // println!("    (triangle {})", triangle);
    }

    if bad_normals > 0 {
        println!("File had {} incorrect normals", bad_normals);
    }

    let inters: Vec<Vec<Cvector>> = triangles.iter().map(|t| Ray::from_tri(t).all_intersections(&triangles)).collect();

    println!("");
    for i in 0..triangles.len() {
        if inters[i].len() % 2 != 0 {
            println!("{}", triangles[i]);
            for p in inters[i].clone() {
                println!("  {}", p);
            }
            println!("Broken mesh!!!");
            break;
        }
    }

    // println!();
    // for t in &triangles {
    //     println!("Triangle {}", t);
    //     let r = Ray::from_tri(&t);
    //     println!("  Ray {}", r);
    //     for t2 in &triangles {
    //         let inter = r.intersect(&t2);
    //         match inter {
    //             Some(p) => println!("    Intersects {}\n      at {}", t2, p),
    //             None => (),
    //         }
    //     }
    // }



    // println!("  )");

    // println!(")");
}


#[cfg(test)]
mod tests {
    use read_binary_stl;

    #[test]
    fn cube() {
        read_binary_stl("./test_data/cube.stl");
    }

    #[test]
    fn sphere() {
        read_binary_stl("./test_data/sphere.stl");
    }

    #[test]
    fn good_bunny() {
        read_binary_stl("./test_data/good_bunny.stl");
    }

    #[test]
    fn reversed_bunny() {
        read_binary_stl("./test_data/reversed_bunny.stl");
    }

    #[test]
    fn disconnected_bunny() {
        read_binary_stl("./test_data/disconnected_bunny.stl");
    }

    #[test]
    fn hole_bunny() {
        read_binary_stl("./test_data/hole_bunny.stl");
    }

    #[test]
    fn different_bunny() {
        read_binary_stl("./test_data/different_bunny.stl");
    }

}
