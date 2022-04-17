use nalgebra::vector;
use nalgebra::Vector3;

use crate::{Camera, Scene, Triangle, ViewPlane};

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_triangles() -> Vec<Triangle> {
    let mut vertices: Vec<Vector3<f64>> = vec![];
    let mut triangles: Vec<Triangle> = vec![];
    if let Ok(lines) = read_lines("./src/Rabbit_Lowpoly_1.obj") {
        for line in lines {
            if let Ok(ip) = line {
                let info = ip.split(" ").collect::<Vec<&str>>();
                match info[0] {
                    "v" => vertices.push(vector![
                        info[1].parse().unwrap(),
                        info[2].parse().unwrap(),
                        info[3].parse().unwrap()
                    ]),
                    "f" => {
                        let idx0: usize = info[1].split("//").collect::<Vec<&str>>()[0]
                            .parse()
                            .unwrap();
                        let idx1: usize = info[2].split("//").collect::<Vec<&str>>()[0]
                            .parse()
                            .unwrap();
                        let idx2: usize = info[3].split("//").collect::<Vec<&str>>()[0]
                            .parse()
                            .unwrap();
                        triangles.push(Triangle::new(
                            vertices[idx0 - 1],
                            vertices[idx1 - 1],
                            vertices[idx2 - 1],
                        ))
                    }
                    _ => (),
                }
            }
        }
    }
    triangles
}

pub fn run() -> Scene {
    let camera = Camera {
        location: vector![2.0, 2.0, 2.0],
        roll: -0.5,
        pitch: 2.0,
        yaw: 3.1415,
    };
    let view_plane = ViewPlane::new(&camera, 1.0, 0.025, 120);
    Scene {
        triangles: get_triangles(),
        view_plane,
    }
}
