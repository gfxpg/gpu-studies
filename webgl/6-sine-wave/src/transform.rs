pub fn matmul4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut out: [f32; 16] = [0.0; 16];

    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                out[i * 4 + j] += a[i * 4 + k] * b[k * 4 + j];
            }
        }
    }

    out
}

pub fn mat4_rot_x(rad: f32) -> [f32; 16] {
    [
        1.0, 0.0,        0.0,       0.0,
        0.0, rad.cos(),  rad.sin(), 0.0,
        0.0, -rad.sin(), rad.cos(), 0.0,
        0.0, 0.0,        0.0,       1.0
    ]
}

pub fn mat4_rot_y(rad: f32) -> [f32; 16] {
    [
        rad.cos(), 0.0, -rad.sin(), 0.0,
        0.0,       1.0, 0.0,        0.0,
        rad.sin(), 0.0, rad.cos(),  0.0,
        0.0,       0.0, 0.0,        1.0
    ]
}

pub fn mat4_scale(s: f32) -> [f32; 16] {
    [
        s,   0.0, 0.0, 0.0,
        0.0, s,   0.0, 0.0,
        0.0, 0.0, s,   0.0,
        0.0, 0.0, 0.0, 1.0
    ]
}

pub fn mat4_perspective(field_of_view_rad: f32, aspect: f32, z_near: f32, z_far: f32) -> [f32; 16] {
    let f = (std::f32::consts::FRAC_PI_2 - 0.5 * field_of_view_rad).tan();
    let range_inv = 1.0 / (z_near - z_far);

    [
        f / aspect, 0.0, 0.0,                              0.0,
        0.0,        f,   0.0,                              0.0,
        0.0,        0.0, (z_near + z_far) * range_inv,     -1.0,
        0.0,        0.0, z_near * z_far * range_inv * 2.0, 0.0
    ]
}

pub fn mat4_translation(x: f32, y: f32, z: f32) -> [f32; 16] {
    [
         1.0,  0.0,  0.0,  0.0,
         0.0,  1.0,  0.0,  0.0,
         0.0,  0.0,  1.0,  0.0,
         x,    y,    z,    1.0
    ]
}
