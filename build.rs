const PROTOS: &[&str] = &["src/protos/fileformat.proto", "src/protos/osmformat.proto"];

fn main() {
    for proto in PROTOS {
        println!("cargo:rerun-if-changed={}", proto);
    }
    prost_build::compile_protos(PROTOS, &["src/protos"]).unwrap();
}
