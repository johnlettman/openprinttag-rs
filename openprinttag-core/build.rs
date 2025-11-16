use openprinttag_codegen::{
    loader::GitHubLoader,
    schema::{context::Context, gen::ToFile, ConfigStructSchema},
};
use std::{fs, path::Path, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let loader = GitHubLoader::new();
    let context = Context::new(Box::new(loader));

    let config = ConfigStructSchema::load(context, "config_nfcv").expect("should load schema");
    let out = Path::new("src/lib.rs");
    let generated = config.to_core_file_string();

    fs::write(out, &generated).expect("failed to write src/lib.rs");

    // ---- 3. Run rustfmt (using shell) ----
    let status = Command::new("rustfmt")
        .arg(out.to_str().unwrap())
        .status()
        .expect("failed to execute rustfmt");

    if !status.success() {
        panic!("rustfmt failed with status: {}", status);
    }
}
