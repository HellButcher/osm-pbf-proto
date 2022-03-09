fn main() {
    protobuf_codegen::Codegen::new()
        .pure()
        .cargo_out_dir("protos-gen")
        .includes(&["src/protos"])
        .inputs(&["src/protos/fileformat.proto", "src/protos/osmformat.proto"])
        .run_from_script();
}
