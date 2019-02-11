use std::{env, fs};
use std::path::{Path, PathBuf};
use shaderc;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut shaders_dir = env::current_dir().unwrap();
    shaders_dir.push("shaders");

    let shader_paths: Vec<PathBuf> = fs::read_dir(shaders_dir).unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.is_file() && match path.extension().and_then(|e| e.to_str()) {
                Some("vert") | Some("frag") => true,
                _ => false
            }
        })
        .collect();

    for shader_path in shader_paths {
        let source = fs::read_to_string(&shader_path).unwrap();
        let shader_kind = match shader_path.extension().and_then(|e| e.to_str()).unwrap() {
            "vert" => shaderc::ShaderKind::Vertex,
            "frag" => shaderc::ShaderKind::Fragment,
            _ => unreachable!()
        };
        let shader_filename = shader_path.file_name().and_then(|n| n.to_str()).unwrap();

        let mut compiler = shaderc::Compiler::new().unwrap();
        let shader_spirv = compiler.compile_into_spirv(
            &source, shader_kind, shader_filename, "main", None
        ).unwrap();

        let dest_path = Path::new(&out_dir).join(format!("{}.spv", shader_filename));

        fs::write(dest_path, shader_spirv.as_binary_u8()).unwrap();
    }
}
