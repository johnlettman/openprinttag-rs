use std::path::Path;
use openprinttag_codegen::loader::GitHubLoader;
use openprinttag_codegen::schema::{ConfigStructSchema, GetSchemas};
use openprinttag_codegen::schema::context::Context;
use openprinttag_codegen::schema::gen::GetSchemaName;
use crate::command::Command;

#[derive(Debug, Clone, clap::Args)]
pub struct Inspect {

}

impl Command for Inspect {
    fn run(&self) -> crate::Result<()> {
        let loader = GitHubLoader::new();
        let context = Context::new(Box::new(loader));

        let config = ConfigStructSchema::load(context, "config_nfcv").expect("should load schema");

        for schema in config.get_schemas() {
            println!("Schema: {}", schema.get_schema_name());
            println!("{:#?}", schema);
        }
        println!("{:#?}", config);
        Ok(())
    }
}
