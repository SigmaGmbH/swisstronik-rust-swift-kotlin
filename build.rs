extern crate protobuf_codegen;

fn main() {
    protobuf_codegen::Codegen::new()
        .out_dir("src/protobuf_generated")
        .includes(&["./protobuf_contracts"])
        .inputs(&["./protobuf_contracts/contract.proto"])
        .run_from_script();
}