use protobuf_codegen_pure::{Codegen, Customize};

fn main() {
    if let Err(e) = Codegen::new()
        .out_dir("src/protos")
        .inputs(&["protos/token.proto"])
        .includes(&["protos"])
        .customize(Customize {
            ..Default::default()
        })
        .run()
    {
        println!(
            "cargo:warning=fail to generate protobuf file from source: {:?}",
            e
        );
    }
}
