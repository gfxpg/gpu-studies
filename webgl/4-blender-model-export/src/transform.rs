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
