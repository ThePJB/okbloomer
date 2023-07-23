use crate::vector::*;

pub fn view_mat(pos: Vec3, dir: Vec3) -> [f32; 16] {
    let zaxis = (pos - dir).normalize();
    let xaxis = Vec3 { x: 0.0, y: 1.0, z: 0.0 }.cross(zaxis).normalize();
    let yaxis = zaxis.cross(xaxis).normalize();
    [
        xaxis.x, yaxis.x, zaxis.x, 0.0,
        xaxis.y, yaxis.y, zaxis.y, 0.0,
        xaxis.z, yaxis.z, zaxis.z, 0.0,
        -xaxis.dot(pos), -yaxis.dot(pos), -zaxis.dot(pos), 1.0,
    ]
}

// fov in radians
fn projection_matrix(fov: f32, aspect: f32, z_near: f32, z_far: f32) -> [f32; 16] {
    let aspect_ratio = 1.0; // Replace this with your actual aspect ratio if needed

    let tan_half_fov = (fov / 2.0).tan();
    let z_range = z_near - z_far;

    [
        1.0 / (aspect * tan_half_fov), 0.0, 0.0, 0.0,
        0.0, 1.0 / tan_half_fov, 0.0, 0.0,
        0.0, 0.0, (z_near + z_far) / z_range, 2.0 * z_far * z_near / z_range,
        0.0, 0.0, -1.0, 0.0,
    ]
}

fn mat_mul4(a: &[f32; 16], b: &[f32; 16]) -> [f32; 16] {
    [
        a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12],
        a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13],
        a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14],
        a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15],

        a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12],
        a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13],
        a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14],
        a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15],

        a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12],
        a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13],
        a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14],
        a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15],

        a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12],
        a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13],
        a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14],
        a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15],
    ]
}

pub fn look_at(cam_pos: Vec3, cam_dir: Vec3, fov: f32, aspect: f32, z_near: f32, z_far: f32) -> [f32; 16] {
    let view_matrix = view_mat(cam_pos, cam_dir);
    let projection_matrix = projection_matrix(fov, aspect, z_near, z_far);
    mat_mul4(&projection_matrix, &view_matrix)
}