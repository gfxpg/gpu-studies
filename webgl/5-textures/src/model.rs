use ply_rs::ply;
use ply_rs::parser;

pub struct Model {
    pub vertices: Vec<f32>,
    pub texcoords: Vec<f32>
}

struct Vertex { x: f32, y: f32, z: f32, s: f32, t: f32 }

struct Face { vertex_indices: Vec<u32> }

impl ply::PropertyAccess for Vertex {
    fn new() -> Self {
        Vertex { x: 0.0, y: 0.0, z: 0.0, s: 0.0, t: 0.0 }
    }

    fn set_property(&mut self, key: String, property: ply::Property) {
        match (key.as_ref(), property) {
            ("x", ply::Property::Float(v)) => self.x = v,
            ("y", ply::Property::Float(v)) => self.y = v,
            ("z", ply::Property::Float(v)) => self.z = v,
            ("s", ply::Property::Float(v)) => self.s = v,
            ("t", ply::Property::Float(v)) => self.t = v,
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

pub fn load(ascii_source: &str) -> std::io::Result<Model> {
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

    let texcoord_array: Vec<f32> = faces.iter().flat_map(|f| {
        f.vertex_indices.iter().flat_map(|&i| vec![
            vertices[i as usize].s, vertices[i as usize].t
        ])
    }).collect();

    Ok(Model { vertices: vertex_array, texcoords: texcoord_array })
}
