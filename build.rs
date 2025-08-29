use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("gsdx_v1_descriptor.bin"))
        .compile_protos(&["proto/gsdx/v1/gsdx.proto"], &["proto"])?;
    Ok(())
}
