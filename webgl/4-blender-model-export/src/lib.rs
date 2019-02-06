use wasm_bindgen::prelude::*;

use wavefront_obj::obj;

#[wasm_bindgen]
pub fn load_vertices(obj_src: String, object_idx: usize) -> Vec<f32> {
    let obj_set = obj::parse(obj_src).unwrap();
    let obj = &obj_set.objects[object_idx];

    obj.vertices
        .iter()
        .flat_map(|&obj::Vertex { x, y, z }| vec![x as f32, y as f32, z as f32])
        .collect()
}
