use openprinttag_codegen::{
    emit::EmitFile,
    loader::GitHubLoader,
    schema::{context::Context, DataPrintTag},
};
use std::{fs, path::Path, process::Command};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const LOG_ENV: &str = "OPT_BUILD_LOG";

fn main() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_env(LOG_ENV)
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("trace"));

    #[cfg(feature = "build-tracing-tree")]
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_forest::ForestLayer::default())
        .init();

    #[cfg(not(feature = "build-tracing-tree"))]
    tracing_subscriber::fmt()
        .with_writer(tracing_build_script::BuildScriptMakeWriter)
        .with_env_filter(env_filter)
        .compact()
        .init();

    println!("cargo:rerun-if-changed=build.rs");

    let loader = GitHubLoader::new();
    let context = Context::new(Box::new(loader));

    let config = DataPrintTag::load(context, "config_nfcv").expect("failed to load schema");
    let out = Path::new("src/gen.rs");
    let generated = config.emit_core_file_string();

    fs::write(out, &generated).expect("failed to write src/gen.rs");

    // ---- 3. Run rustfmt (using shell) ----
    let status = Command::new("rustfmt")
        .arg(out.to_str().unwrap())
        .status()
        .expect("failed to execute rustfmt");

    if !status.success() {
        panic!("rustfmt failed with status: {}", status);
    }
}
