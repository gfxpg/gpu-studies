use ply_rs::ply;
use ply_rs::parser;

#[derive(Debug)]
struct Vertex { x: f32, y: f32, z: f32, r: u8, g: u8, b: u8 }

#[derive(Debug)]
struct Face { vertex_indices: Vec<u32> }

impl ply::PropertyAccess for Vertex {
    fn new() -> Self {
        Vertex { x: 0.0, y: 0.0, z: 0.0, r: 0, g: 0, b: 0 }
    }

    fn set_property(&mut self, key: String, property: ply::Property) {
        match (key.as_ref(), property) {
            ("x", ply::Property::Float(v)) => self.x = v,
            ("y", ply::Property::Float(v)) => self.y = v,
            ("z", ply::Property::Float(v)) => self.z = v,
            ("red", ply::Property::UChar(v)) => self.r = v,
            ("green", ply::Property::UChar(v)) => self.g = v,
            ("blue", ply::Property::UChar(v)) => self.b = v,
            _ => ()
        }
    }
}

impl ply::PropertyAccess for Face {
    fn new() -> Self {
        Face { vertex_indices: Vec::new() }
    }

    fn set_property(&mut self, key: String, property: ply::Property) {
        if let ("vertex_indices", ply::Property::ListUInt(vec)) = (key.as_ref(), property) {
            self.vertex_indices = vec;
        }
    }
}

pub fn load(ascii_source: &str) -> std::io::Result<(Vec<f32>, Vec<u8>)> {
    let mut model_source = std::io::Cursor::new(ascii_source);
    let vertex_parser = parser::Parser::<Vertex>::new();
    let face_parser = parser::Parser::<Face>::new();

    let header = vertex_parser.read_header(&mut model_source)?;
    let vertices = vertex_parser.read_payload_for_element(&mut model_source, &header.elements["vertex"], &header)?;
    let faces = face_parser.read_payload_for_element(&mut model_source, &header.elements["face"], &header)?;

    let vertex_array: Vec<f32> = faces.iter().flat_map(|f| {
        f.vertex_indices.iter().flat_map(|&i| vec![
            vertices[i as usize].x, vertices[i as usize].y, vertices[i as usize].z
        ])
    }).collect();

    let color_array: Vec<u8> = faces.iter().flat_map(|f| {
        f.vertex_indices.iter().flat_map(|&i| vec![
            vertices[i as usize].r, vertices[i as usize].g, vertices[i as usize].b
        ])
    }).collect();

    Ok((vertex_array, color_array))
}
