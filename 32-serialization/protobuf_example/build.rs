use prost_build::Config;
use std::{io::Result, path::Path};

fn main() -> Result<()> {
    let mut config = Config::new();

    let path = Path::new("out");
    std::fs::create_dir_all(path).expect("failed to create output directory");
    config.out_dir(path);

    config.compile_protos(&["voting.proto"], &["."])?;
    Ok(())
}
