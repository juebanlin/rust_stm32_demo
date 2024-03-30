fn main() {
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/proto_msg")
        .inputs(&["protos/ProtoMsg.proto"])
        .include("protos")
        .run()
        .expect("Codegen failed.");
}