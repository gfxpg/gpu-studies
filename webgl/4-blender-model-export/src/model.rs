use ply_rs as ply;

pub fn load(ascii_source: &str) -> std::io::Result<(Vec<f32>, Vec<u8>)> {
    use ply_rs::ply::PropertyAccess;

    let parser = ply::parser::Parser::<ply::ply::DefaultElement>::new();
    let mut model_source = std::io::Cursor::new(ascii_source);
    let model = parser.read_ply(&mut model_source)?.payload;

    let x_key = "x".to_owned();
    let y_key = "y".to_owned();
    let z_key = "z".to_owned();

    let r_key = "red".to_owned();
    let g_key = "green".to_owned();
    let b_key = "blue".to_owned();

    let vertices: Vec<f32> = model["vertex"].iter().flat_map(|m| {
        vec![
            m.get_float(&x_key).unwrap(),
            m.get_float(&y_key).unwrap(),
            m.get_float(&z_key).unwrap()
        ]
    }).collect();

    let colors: Vec<u8> = model["vertex"].iter().flat_map(|m| {
        vec![
            m.get_uchar(&r_key).unwrap(),
            m.get_uchar(&g_key).unwrap(),
            m.get_uchar(&b_key).unwrap()
        ]
    }).collect();

    Ok((vertices, colors))
}
