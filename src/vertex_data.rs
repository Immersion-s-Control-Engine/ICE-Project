#![allow(dead_code)]

use cgmath::*;
#[path = "../src/math_func.rs"]
mod math_func;

/// Shape function for a torus.
pub fn torus_data(
    r_torus: f32,
    r_tube: f32,
    n_torus: usize,
    n_tube: usize,
) -> (Vec<[f32; 3]>, Vec<[f32; 3]>) {
    let mut positions: Vec<[f32; 3]> =
        Vec::with_capacity((4 * (n_torus - 1) * (n_tube - 1)) as usize);
    let mut normals: Vec<[f32; 3]> =
        Vec::with_capacity((4 * (n_torus - 1) * (n_tube - 1)) as usize);

    for i in 0..n_torus - 1 {
        for j in 0..n_tube - 1 {
            let u = i as f32 * 360.0 / (n_torus as f32 - 1.0);
            let v = j as f32 * 360.0 / (n_tube as f32 - 1.0);
            let u1 = (i as f32 + 1.0) * 360.0 / (n_torus as f32 - 1.0);
            let v1 = (j as f32 + 1.0) * 360.0 / (n_tube as f32 - 1.0);
            let p0 = math_func::torus_position(r_torus, r_tube, Deg(u), Deg(v));
            let p1 = math_func::torus_position(r_torus, r_tube, Deg(u1), Deg(v));
            let p2 = math_func::torus_position(r_torus, r_tube, Deg(u1), Deg(v1));
            let p3 = math_func::torus_position(r_torus, r_tube, Deg(u), Deg(v1));

            // positions
            positions.push(p0);
            positions.push(p1);
            positions.push(p2);
            positions.push(p2);
            positions.push(p3);
            positions.push(p0);

            // normals
            let ca = Vector3::new(p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]);
            let db = Vector3::new(p3[0] - p1[0], p3[1] - p1[1], p3[2] - p1[2]);
            let cp = (ca.cross(db)).normalize();

            normals.push([cp[0], cp[1], cp[2]]);
            normals.push([cp[0], cp[1], cp[2]]);
            normals.push([cp[0], cp[1], cp[2]]);
            normals.push([cp[0], cp[1], cp[2]]);
            normals.push([cp[0], cp[1], cp[2]]);
            normals.push([cp[0], cp[1], cp[2]]);
        }
    }
    (positions, normals)
}

/// Shape function for a sphere.
pub fn sphere_data(r: f32, u: usize, v: usize) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>) {
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity((4 * (u - 1) * (v - 1)) as usize);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity((4 * (u - 1) * (v - 1)) as usize);
    let uvs: Vec<[f32; 2]> = Vec::with_capacity((4 * (u - 1) * (v - 1)) as usize);

    for i in 0..u - 1 {
        for j in 0..v - 1 {
            let theta = i as f32 * 180.0 / (u as f32 - 1.0);
            let phi = j as f32 * 360.0 / (v as f32 - 1.0);
            let theta1 = (i as f32 + 1.0) * 180.0 / (u as f32 - 1.0);
            let phi1 = (j as f32 + 1.0) * 360.0 / (v as f32 - 1.0);
            let p0 = math_func::sphere_position(r, Deg(theta), Deg(phi));
            let p1 = math_func::sphere_position(r, Deg(theta1), Deg(phi));
            let p2 = math_func::sphere_position(r, Deg(theta1), Deg(phi1));
            let p3 = math_func::sphere_position(r, Deg(theta), Deg(phi1));

            // positions
            positions.push(p0);
            positions.push(p1);
            positions.push(p3);
            positions.push(p1);
            positions.push(p2);
            positions.push(p3);

            // normals
            normals.push([p0[0] / r, p0[1] / r, p0[2] / r]);
            normals.push([p1[0] / r, p1[1] / r, p1[2] / r]);
            normals.push([p3[0] / r, p3[1] / r, p3[2] / r]);
            normals.push([p1[0] / r, p1[1] / r, p1[2] / r]);
            normals.push([p2[0] / r, p2[1] / r, p2[2] / r]);
            normals.push([p3[0] / r, p3[1] / r, p3[2] / r]);
        }
    }
    (positions, normals, uvs)
}

/// Shape function for a cube.
pub fn cube_positions() -> Vec<[i8; 3]> {
    [
        // front (0, 0, 1)
        [-1, -1, 1],
        [1, -1, 1],
        [-1, 1, 1],
        [-1, 1, 1],
        [1, -1, 1],
        [1, 1, 1],
        // right (1, 0, 0)
        [1, -1, 1],
        [1, -1, -1],
        [1, 1, 1],
        [1, 1, 1],
        [1, -1, -1],
        [1, 1, -1],
        // back (0, 0, -1)
        [1, -1, -1],
        [-1, -1, -1],
        [1, 1, -1],
        [1, 1, -1],
        [-1, -1, -1],
        [-1, 1, -1],
        // left (-1, 0, 0)
        [-1, -1, -1],
        [-1, -1, 1],
        [-1, 1, -1],
        [-1, 1, -1],
        [-1, -1, 1],
        [-1, 1, 1],
        // top (0, 1, 0)
        [-1, 1, 1],
        [1, 1, 1],
        [-1, 1, -1],
        [-1, 1, -1],
        [1, 1, 1],
        [1, 1, -1],
        // bottom (0, -1, 0)
        [-1, -1, -1],
        [1, -1, -1],
        [-1, -1, 1],
        [-1, -1, 1],
        [1, -1, -1],
        [1, -1, 1],
    ]
    .to_vec()
}

pub fn cube_colors() -> Vec<[i8; 3]> {
    [
        // front - blue
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        // right - red
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        // back - yellow
        [1, 1, 0],
        [1, 1, 0],
        [1, 1, 0],
        [1, 1, 0],
        [1, 1, 0],
        [1, 1, 0],
        // left - aqua
        [0, 1, 1],
        [0, 1, 1],
        [0, 1, 1],
        [0, 1, 1],
        [0, 1, 1],
        [0, 1, 1],
        // top - green
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        // bottom - fuchsia
        [1, 0, 1],
        [1, 0, 1],
        [1, 0, 1],
        [1, 0, 1],
        [1, 0, 1],
        [1, 0, 1],
    ]
    .to_vec()
}

pub fn cube_normals() -> Vec<[i8; 3]> {
    [
        // front
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        // right
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        // back
        [0, 0, -1],
        [0, 0, -1],
        [0, 0, -1],
        [0, 0, -1],
        [0, 0, -1],
        [0, 0, -1],
        // left
        [-1, 0, 0],
        [-1, 0, 0],
        [-1, 0, 0],
        [-1, 0, 0],
        [-1, 0, 0],
        [-1, 0, 0],
        // top
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        // bottom
        [0, -1, 0],
        [0, -1, 0],
        [0, -1, 0],
        [0, -1, 0],
        [0, -1, 0],
        [0, -1, 0],
    ]
    .to_vec()
}
