export function rad(deg) {
  return deg * Math.PI / 180;
}

export function matrotx3d(rad) {
  const cos = Math.cos(rad), sin = Math.sin(rad);

  return new Float32Array([
    1, 0,    0,   0,
    0, cos,  sin, 0,
    0, -sin, cos, 0,
    0, 0,    0,   1
  ]);
}

export function matroty3d(rad) {
  const cos = Math.cos(rad), sin = Math.sin(rad); 

  return new Float32Array([
    cos, 0, -sin, 0,
    0,   1, 0,    0,
    sin, 0, cos,  0,
    0,   0, 0,    1
  ]);
}

export function matrotz3d(rad) {
  const cos = Math.cos(rad), sin = Math.sin(rad); 

  return new Float32Array([
    cos,  sin, 0, 0,
    -sin, cos, 0, 0,
    0,    0,   1, 0,
    0,    0,   0, 1
  ]);
}

export function matscale3d(sx, sy, sz) {
  return new Float32Array([
    sx, 0,  0,  0,
    0,  sy, 0,  0,
    0,  0,  sz, 0,
    0,  0,  0,  1,
  ]);
}
