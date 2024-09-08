fn main() {
    protobuf_codegen::Codegen::new()
        .cargo_out_dir("messages")
        .include("src")
        .input("src/proto/messages.proto")
        .input("src/proto/datatypes.proto")
        .run_from_script();
}
