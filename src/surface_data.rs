#![allow(dead_code)]
use cgmath::*;
mod colormap;
use std::f32::consts::PI;
#[path = "./math_func.rs"]
mod math_func;

pub struct ParametricSurface {
    pub f: fn(f32, f32, [f32; 5]) -> [f32; 3],
    pub umin: f32,
    pub umax: f32,
    pub vmin: f32,
    pub vmax: f32,
    pub u_segments: usize,
    pub v_segments: usize,
    pub scale: f32,
    pub aspect: f32,
    pub use_colormap: bool,
    pub colormap_name: &'static str,
    pub colormap_direction: &'static str,
    pub color: [f32; 3],
    pub params: [f32; 5],
}

impl Default for ParametricSurface {
    fn default() -> Self {
        ParametricSurface {
            f: math_func::torus,
            umin: 0.0,
            umax: 2.0 * PI,
            vmin: 0.0,
            vmax: 2.0 * PI,
            u_segments: 32,
            v_segments: 24,
            scale: 1.5,
            aspect: 1.0,
            use_colormap: true,
            colormap_name: "jet",
            colormap_direction: "y",
            color: [1.0, 0.0, 0.0],
            params: [1.0, 0.3, 0.0, 0.0, 0.0],
        }
    }
}

impl ParametricSurface {
    pub fn new(ps: ParametricSurface) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<u32>) {
        let n_vertices = (ps.u_segments + 1) * (ps.v_segments + 1);
        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
        let mut colors: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);

        let du = (ps.umax - ps.umin) / ps.u_segments as f32;
        let dv = (ps.vmax - ps.vmin) / ps.v_segments as f32;

        let eps = 0.00001;
        let mut p0: Vector3<f32>;
        let mut p1: Vector3<f32>;
        let mut p2: Vector3<f32>;
        let mut p3: Vector3<f32>;
        let mut pa: [f32; 3];

        let mut cd = 1;
        if ps.colormap_direction == "x" {
            cd = 0;
        } else if ps.colormap_direction == "z" {
            cd = 2;
        }

        let (min, max) = parametric_surface_range(
            ps.f,
            ps.umin,
            ps.umax,
            ps.vmin,
            ps.vmax,
            ps.u_segments,
            ps.v_segments,
            ps.scale,
            ps.aspect,
            ps.params,
            cd,
        );

        for i in 0..=ps.u_segments {
            let u = ps.umin + i as f32 * du;
            for j in 0..=ps.v_segments {
                let v = ps.vmin + j as f32 * dv;
                pa = (ps.f)(u, v, ps.params);
                p0 = Vector3::new(
                    pa[0] * ps.scale,
                    ps.scale * ps.aspect * pa[1],
                    ps.scale * pa[2],
                );

                // calculate normals
                if u - eps >= 0.0 {
                    pa = (ps.f)(u - eps, v, ps.params);
                    p1 = Vector3::new(
                        pa[0] * ps.scale,
                        ps.scale * ps.aspect * pa[1],
                        ps.scale * pa[2],
                    );
                    p2 = p0 - p1;
                } else {
                    pa = (ps.f)(u + eps, v, ps.params);
                    p1 = Vector3::new(
                        pa[0] * ps.scale,
                        ps.scale * ps.aspect * pa[1],
                        ps.scale * pa[2],
                    );
                    p2 = p1 - p0;
                }

                if v - eps >= 0.0 {
                    pa = (ps.f)(u, v - eps, ps.params);
                    p1 = Vector3::new(
                        pa[0] * ps.scale,
                        ps.scale * ps.aspect * pa[1],
                        ps.scale * pa[2],
                    );
                    p3 = p0 - p1;
                } else {
                    pa = (ps.f)(u, v + eps, ps.params);
                    p1 = Vector3::new(
                        pa[0] * ps.scale,
                        ps.scale * ps.aspect * pa[1],
                        ps.scale * pa[2],
                    );
                    p3 = p1 - p0;
                }
                let normal = p3.cross(p2).normalize();

                // calculate colrmap
                let mut color = ps.color;
                if ps.use_colormap {
                    color = colormap::color_interp(ps.colormap_name, min, max, p0[cd]);
                }

                positions.push(p0.into());
                normals.push(normal.into());
                colors.push(color);
            }
        }

        let n_faces = ps.u_segments * ps.v_segments;
        let n_triangles = n_faces * 2;
        let n_indices = n_triangles * 3;

        let mut indices: Vec<u32> = Vec::with_capacity(n_indices);

        let n_vertices_per_row = ps.v_segments + 1;

        for i in 0..ps.u_segments {
            for j in 0..ps.v_segments {
                let idx0 = j + i * n_vertices_per_row;
                let idx1 = j + 1 + i * n_vertices_per_row;
                let idx2 = j + 1 + (i + 1) * n_vertices_per_row;
                let idx3 = j + (i + 1) * n_vertices_per_row;

                indices.push(idx0 as u32);
                indices.push(idx1 as u32);
                indices.push(idx2 as u32);

                indices.push(idx2 as u32);
                indices.push(idx3 as u32);
                indices.push(idx0 as u32);
            }
        }
        (positions, normals, colors, indices)
    }
}

fn parametric_surface_range(
    f: fn(f32, f32, [f32; 5]) -> [f32; 3],
    umin: f32,
    umax: f32,
    vmin: f32,
    vmax: f32,
    nu: usize,
    nv: usize,
    scale: f32,
    aspect: f32,
    params: [f32; 5],
    dir: usize,
) -> (f32, f32) {
    let du = (umax - umin) / nu as f32;
    let dv = (vmax - vmin) / nv as f32;
    let mut min: f32 = std::f32::MAX;
    let mut max: f32 = std::f32::MIN;

    for i in 0..=nu {
        let u = umin + i as f32 * du;
        for j in 0..=nv {
            let v = vmin + j as f32 * dv;
            let mut pt = f(u, v, params);
            pt = [pt[0] * scale, scale * aspect * pt[1], scale * pt[2]];
            min = if pt[dir] < min { pt[dir] } else { min };
            max = if pt[dir] > max { pt[dir] } else { max };
        }
    }
    return (min, max);
}
pub fn simple_surface_colors(
    pts: &Vec<Vec<[f32; 3]>>,
    nx: usize,
    nz: usize,
    yrange: [f32; 2],
    colormap_name: &str,
) -> Vec<[f32; 3]> {
    let mut colors: Vec<[f32; 3]> = Vec::with_capacity((4 * (nx - 1) * (nz - 1)) as usize);
    for i in 0..nx - 1 {
        for j in 0..nz - 1 {
            let p0 = pts[i][j];
            let p1 = pts[i][j + 1];
            let p2 = pts[i + 1][j + 1];
            let p3 = pts[i + 1][j];

            let c0 = colormap::color_interp(colormap_name, yrange[0], yrange[1], p0[1]);
            let c1 = colormap::color_interp(colormap_name, yrange[0], yrange[1], p1[1]);
            let c2 = colormap::color_interp(colormap_name, yrange[0], yrange[1], p2[1]);
            let c3 = colormap::color_interp(colormap_name, yrange[0], yrange[1], p3[1]);

            colors.push(c0);
            colors.push(c1);
            colors.push(c2);
            colors.push(c2);
            colors.push(c3);
            colors.push(c0);
        }
    }
    colors
}

pub fn simple_surface_normals(pts: &Vec<Vec<[f32; 3]>>, nx: usize, nz: usize) -> Vec<[f32; 3]> {
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity((4 * (nx - 1) * (nz - 1)) as usize);
    for i in 0..nx - 1 {
        for j in 0..nz - 1 {
            let p0 = pts[i][j];
            let p1 = pts[i][j + 1];
            let p2 = pts[i + 1][j + 1];
            let p3 = pts[i + 1][j];

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
    normals
}

pub fn simple_surface_positions(pts: &Vec<Vec<[f32; 3]>>, nx: usize, nz: usize) -> Vec<[f32; 3]> {
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity((4 * (nx - 1) * (nz - 1)) as usize);
    for i in 0..nx - 1 {
        for j in 0..nz - 1 {
            let p0 = pts[i][j];
            let p1 = pts[i][j + 1];
            let p2 = pts[i + 1][j + 1];
            let p3 = pts[i + 1][j];

            positions.push(p0);
            positions.push(p1);
            positions.push(p2);
            positions.push(p2);
            positions.push(p3);
            positions.push(p0);
        }
    }
    positions
}

pub fn simple_surface_points(
    f: &dyn Fn(f32, f32) -> [f32; 3],
    xmin: f32,
    xmax: f32,
    zmin: f32,
    zmax: f32,
    nx: usize,
    nz: usize,
    scale: f32,
    aspect: f32,
) -> (Vec<Vec<[f32; 3]>>, [f32; 2]) {
    let dx = (xmax - xmin) / (nx as f32 - 1.0);
    let dz = (zmax - zmin) / (nz as f32 - 1.0);
    let mut ymin: f32 = 0.0;
    let mut ymax: f32 = 0.0;

    let mut pts: Vec<Vec<[f32; 3]>> = vec![vec![Default::default(); nz]; nx];
    for i in 0..nx {
        let x = xmin + i as f32 * dx;
        let mut pt1: Vec<[f32; 3]> = Vec::with_capacity(nz);
        for j in 0..nz {
            let z = zmin + j as f32 * dz;
            let pt = f(x, z);
            pt1.push(pt);
            ymin = if pt[1] < ymin { pt[1] } else { ymin };
            ymax = if pt[1] > ymax { pt[1] } else { ymax };
        }
        pts[i] = pt1;
    }

    let ymin1 = ymin - (1.0 - aspect) * (ymax - ymin);
    let ymax1 = ymax + (1.0 - aspect) * (ymax - ymin);

    for i in 0..nx {
        for j in 0..nz {
            pts[i][j] = normalize_point(pts[i][j], xmin, xmax, ymin1, ymax1, zmin, zmax, scale);
        }
    }

    let cmin = normalize_point(
        [0.0, ymin, 0.0],
        xmin,
        xmax,
        ymin1,
        ymax1,
        zmin,
        zmax,
        scale,
    )[1];
    let cmax = normalize_point(
        [0.0, ymax, 0.0],
        xmin,
        xmax,
        ymin1,
        ymax1,
        zmin,
        zmax,
        scale,
    )[1];

    return (pts, [cmin, cmax]);
}

fn normalize_point(
    pt: [f32; 3],
    xmin: f32,
    xmax: f32,
    ymin: f32,
    ymax: f32,
    zmin: f32,
    zmax: f32,
    scale: f32,
) -> [f32; 3] {
    let px = scale * (-1.0 + 2.0 * (pt[0] - xmin) / (xmax - xmin));
    let py = scale * (-1.0 + 2.0 * (pt[1] - ymin) / (ymax - ymin));
    let pz = scale * (-1.0 + 2.0 * (pt[2] - zmin) / (zmax - zmin));
    [px, py, pz]
}
