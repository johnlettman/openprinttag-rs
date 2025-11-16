use std::path::Path;
use openprinttag_codegen::loader::GitHubLoader;
use openprinttag_codegen::schema::ConfigStructSchema;
use openprinttag_codegen::schema::context::Context;
use openprinttag_codegen::schema::gen::ToFile;

#[derive(Debug, Clone, clap::Args)]
pub struct Gen {

}

impl crate::command::Command for Gen {
    fn run(&self) -> crate::Result<()> {
        let loader = GitHubLoader::new();
        let context = Context::new(Box::new(loader));

        let config = ConfigStructSchema::load(context, "config_nfcv").expect("should load schema");
        let out = Path::new("src/lib.rs");
        let generated = config.to_core_file_string();

        println!("{}", generated);

        Ok(())
    }
}
