mod intersect;
mod render;
mod setup;

use nalgebra::{vector, Rotation3, Vector3};
use sdl2::pixels::Color;
use std::time::SystemTime;

pub struct Scene {
    triangles: Vec<Triangle>,
    view_plane: ViewPlane,
}

pub struct Triangle {
    t1: Vector3<f64>,
    t2: Vector3<f64>,
    t3: Vector3<f64>,
    plane: Plane,
}

impl Triangle {
    fn new(t1: Vector3<f64>, t2: Vector3<f64>, t3: Vector3<f64>) -> Self {
        let a_bar = &t2 - &t1;
        let b_bar = &t3 - &t1;
        let normal = &a_bar.cross(&b_bar);

        let k = -normal.dot(&t1);

        Triangle {
            t1,
            t2,
            t3,
            plane: Plane {
                a: normal.x,
                b: normal.y,
                c: normal.z,
                k,
            },
        }
    }
}

pub struct Plane {
    a: f64,
    b: f64,
    c: f64,
    k: f64,
}

impl Plane {
    pub fn dot(&self, vec: &Vector3<f64>) -> f64 {
        vector![self.a, self.b, self.c].dot(vec)
    }
}

pub struct Camera {
    location: Vector3<f64>,
    roll: f64,
    pitch: f64,
    yaw: f64,
}

impl Camera {
    pub fn orient(&self, ray: &mut Ray) {
        let rot = Rotation3::from_euler_angles(self.roll, self.pitch, self.yaw);
        ray.slope = rot * ray.slope;
        ray.origin = self.location + ray.origin;
    }
}

pub struct ViewPlane {
    pixels: Vec<Vec<Pixel>>,
}

impl ViewPlane {
    pub fn new(camera: &Camera, distance: f64, pixel_width: f64, pixel_side_count: u32) -> Self {
        let mut pixels = vec![];

        for px in 0..pixel_side_count {
            let mut pixel_row = vec![];
            for py in 0..pixel_side_count {
                let mut ray = Ray {
                    slope: vector![
                        pixel_width * (px as i32 - (pixel_side_count / 2) as i32) as f64,
                        pixel_width * (py as i32 - (pixel_side_count / 2) as i32) as f64,
                        distance
                    ],
                    origin: vector![0.0, 0.0, 0.0],
                };
                camera.orient(&mut ray);
                pixel_row.push(Pixel {
                    ray,
                    color: Color::RGBA(0, 0, 0, 0),
                })
            }
            pixels.push(pixel_row);
        }
        ViewPlane { pixels }
    }
}

pub struct Pixel {
    ray: Ray,
    color: Color,
}

pub struct Ray {
    slope: Vector3<f64>,
    origin: Vector3<f64>,
}

fn main() {
    let start = SystemTime::now();
    let mut scene = setup::run();
    println!("{:?}", start.elapsed());
    intersect::run(&scene.triangles, &mut scene.view_plane);
    println!("{:?}", start.elapsed());
    render::run(&scene.view_plane);
    println!("{:?}", start.elapsed());
}
