#![allow(dead_code)]
use cgmath::*;

/// Parametric function to draw klein bottle model.
pub fn klein_bottle(u: f32, v: f32, _params: [f32; 5]) -> [f32; 3] {
    let x = 2.0 / 15.0 * (3.0 + 5.0 * u.cos() * u.sin()) * v.sin();

    let y = -1.0 / 15.0
        * u.sin()
        * (3.0 * v.cos()
            - 3.0 * (u.cos()).powf(2.0) * v.cos()
            - 48.0 * (u.cos()).powf(4.0) * v.cos()
            + 48.0 * (u.cos()).powf(6.0) * v.cos()
            - 60.0 * u.sin()
            + 5.0 * u.cos() * v.cos() * u.sin()
            - 5.0 * (u.cos()).powf(3.0) * v.cos() * u.sin()
            - 80.0 * (u.cos()).powf(5.0) * v.cos() * u.sin()
            + 80.0 * (u.cos()).powf(7.0) * v.cos() * u.sin())
        - 2.0;

    let z = -2.0 / 15.0
        * u.cos()
        * (3.0 * v.cos() - 30.0 * u.sin() + 90.0 * (u.cos()).powf(4.0) * u.sin()
            - 60.0 * (u.cos()).powf(6.0) * u.sin()
            + 5.0 * u.cos() * v.cos() * u.sin());

    [x, y, z]
}

/// Parametric function to draw wellenkugel model
pub fn wellenkugel(u: f32, v: f32, _params: [f32; 5]) -> [f32; 3] {
    let x = u * (u.cos()).cos() * v.sin();
    let y = u * (u.cos()).sin();
    let z = u * (u.cos()).cos() * v.cos();
    [x, y, z]
}

/// Parametric function of creating a torus model
pub fn torus(u: f32, v: f32, params: [f32; 5]) -> [f32; 3] {
    let x = (params[0] + params[1] * v.cos()) * u.cos();
    let y = params[1] * v.sin();
    let z = (params[0] + params[1] * v.cos()) * u.sin();
    [x, y, z]
}

/// Used to create peaks surface.
pub fn peaks(x: f32, z: f32) -> [f32; 3] {
    let y = 3.0 * (1.0 - x) * (1.0 - x) * (-(x * x) - (z + 1.0) * (z + 1.0)).exp()
        - 10.0 * (x / 5.0 - x * x * x - z * z * z * z * z) * (-x * x - z * z).exp()
        - 1.0 / 3.0 * (-(x + 1.0) * (x + 1.0) - z * z).exp();
    [x, y, z]
}

/// Used to create a sinc surface.
pub fn sinc(x: f32, z: f32) -> [f32; 3] {
    let r = (x * x + z * z).sqrt();
    let y = if r == 0.0 { 1.0 } else { r.sin() / r };
    [x, y, z]
}

/// Math functions to get the x, y and z values of a torus.
pub fn torus_position(r_torus: f32, r_tube: f32, u: Deg<f32>, v: Deg<f32>) -> [f32; 3] {
    let x = (r_torus + r_tube * v.cos()) * u.cos();
    let y = r_tube * v.sin();
    let z = -(r_torus + r_tube * v.cos()) * u.sin();
    [x, y, z]
}

/// Math functions to get the x, y and z values of a sphere.
pub fn sphere_position(r: f32, theta: Deg<f32>, phi: Deg<f32>) -> [f32; 3] {
    let x = r * theta.sin() * phi.cos();
    let y = r * theta.cos();
    let z = -r * theta.sin() * phi.sin();
    [x, y, z]
}
