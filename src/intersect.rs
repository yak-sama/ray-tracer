use nalgebra::Vector3;
use sdl2::pixels::Color;

use crate::{Plane, Ray, Triangle, ViewPlane};

fn on_same_side(
    i_bar: &Vector3<f64>,
    t: &Vector3<f64>,
    p1: &Vector3<f64>,
    p2: &Vector3<f64>,
) -> bool {
    let line = p1 - p2;
    let a_bar = line.cross(&(i_bar - p2));
    let b_bar = line.cross(&(t - p2));
    a_bar.dot(&b_bar) >= 0.0
}

fn intersects(point: &Vector3<f64>, triangle: &Triangle) -> bool {
    on_same_side(&point, &triangle.t1, &triangle.t2, &triangle.t3)
        && on_same_side(&point, &triangle.t2, &triangle.t1, &triangle.t3)
        && on_same_side(&point, &triangle.t3, &triangle.t1, &triangle.t2)
}

fn intersection(ray: &Ray, plane: &Plane) -> Option<Vector3<f64>> {
    let dividend = plane.dot(&ray.slope);
    if dividend == 0.0 {
        return None;
    }
    let lambda = -(plane.dot(&ray.origin) + plane.k) / dividend;
    if lambda < 0.0 {
        return None;
    }
    Some(&ray.slope * lambda + ray.origin.clone())
}

fn distance_to_intersection(ray: &Ray, triangle: &Triangle, plane: &Plane) -> f64 {
    if let Some(i) = intersection(ray, plane) {
        if intersects(&i, triangle) {
            return ((i.x - ray.origin.x).powf(2.0)
                + (i.y - ray.origin.y).powf(2.0)
                + (i.z - ray.origin.z).powf(2.0))
            .sqrt();
        }
    }
    std::f64::MAX
}

pub fn run(triangles: &Vec<Triangle>, view_plane: &mut ViewPlane) {
    for pixel in view_plane.pixels.iter_mut().flat_map(|row| row.iter_mut()) {
        let mut distance = std::f64::MAX;
        for triangle in triangles {
            let distance_to_intersection =
                distance_to_intersection(&pixel.ray, triangle, &triangle.plane);
            if distance > distance_to_intersection {
                distance = distance_to_intersection
            }
        }
        let multiple = (255.0 * (1.0 - 2.0 / distance)) as u8;

        pixel.color = Color::RGBA(multiple, multiple, multiple, 0);
    }
}
