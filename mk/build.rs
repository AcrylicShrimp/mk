use std::env::{var as env_var, VarError};

fn main() -> Result<(), VarError> {
    if env_var("CARGO_FEATURE_USE_FS_LOADER").is_ok() {
        println!("cargo:rustc-cfg=feature=\"fs_loader\"");
    } else {
        println!("cargo:rustc-cfg=feature=\"asset_loader\"");
    }

    Ok(())
}
