use pb_rs::{types::FileDescriptor, ConfigBuilder};
use std::path::{Path, PathBuf};

const PROTOS: &[&str] = &["src/protos/fileformat.proto", "src/protos/osmformat.proto"];

fn main() {
    for proto in PROTOS {
        println!("cargo:rerun-if-changed={}", proto);
    }
    let in_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = Path::new(&std::env::var("OUT_DIR").unwrap()).join("protos-gen");
    let protos: Vec<_> = PROTOS.iter().map(|p| in_dir.join(p)).collect();
    let includes: Vec<_> = ["src/protos"].iter().map(PathBuf::from).collect();
    std::fs::create_dir_all(&out_dir).unwrap();
    let config_builder = ConfigBuilder::new(&protos, None, Some(&out_dir), &includes)
        .unwrap()
        .add_deprecated_fields(true);
    FileDescriptor::run(&config_builder.build()).unwrap();
}
