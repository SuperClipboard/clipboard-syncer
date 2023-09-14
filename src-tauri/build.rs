use std::fs;

static OUT_DIR: &str = "src/proto-gen";

fn main() {
    build_proto();

    tauri_build::build()
}

fn build_proto() {
    let protos = ["proto/sync.proto"];

    fs::create_dir_all(OUT_DIR).unwrap();
    tonic_build::configure()
        .build_server(true)
        .out_dir(OUT_DIR)
        .compile(&protos, &["proto/"]).unwrap();

    rerun(&protos);
}

fn rerun(proto_files: &[&str]) {
    for proto_file in proto_files {
        println!("cargo:rerun-if-changed={}", proto_file);
    }
}
