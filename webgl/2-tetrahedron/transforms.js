export function matXRot(deg) {
  const rad = deg * Math.PI / 180, cos = Math.cos(rad), sin = Math.sin(rad);

  return [
    1, 0,    0,   0,
    0, cos,  sin, 0,
    0, -sin, cos, 0,
    0, 0,    0,   1
  ];
}

export function matYRot(deg) {
  const rad = deg * Math.PI / 180, cos = Math.cos(rad), sin = Math.sin(rad); 
  return [
    cos, 0, -sin, 0,
    0,   1, 0,    0,
    sin, 0, cos,  0,
    0,   0, 0,    1
  ];
}

export function matZRot(deg) {
  const rad = deg * Math.PI / 180, cos = Math.cos(rad), sin = Math.sin(rad); 

  return [
    cos,  sin, 0, 0,
    -sin, cos, 0, 0,
    0,    0,   1, 0,
    0,    0,   0, 1
  ];
}

