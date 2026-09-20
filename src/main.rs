mod blueprint;
mod error;
mod pipeline;
mod pyexscrloader;

use blueprint::EmberBlueprint;
use error::EmberError;
use pyexscrloader::PyExScrLoader;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), EmberError> {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match command {
        "build" => {
            let manifest_raw = fs::read_to_string(".anvil")?;
            let blueprint: EmberBlueprint = toml::from_str(&manifest_raw)?;
            pipeline::execute_build(&blueprint)?;
        }
        "seal" | "forge" => {
            let manifest_raw = fs::read_to_string(".anvil")?;
            let blueprint: EmberBlueprint = toml::from_str(&manifest_raw)?;
            pipeline::execute_build(&blueprint)?;
            pipeline::seal_artifact(&blueprint)?;
        }
        "clean" | "recede" => {
            if Path::new(".anvil").exists() {
                let manifest_raw = fs::read_to_string(".anvil")?;
                let blueprint: EmberBlueprint = toml::from_str(&manifest_raw)?;
                pipeline::remove_workspace(&blueprint.build.purge_paths)?;
            } else {
                pipeline::remove_workspace(&[PathBuf::from("target/")])?;
            }
        }
        "run-ext" => {
            if args.len() < 3 {
                println!("Usage: ember run-ext <module-name> [args...]");
                return Ok(());
            }
            let module_name = &args[2];
            let ext_args: Vec<&str> = args.iter().skip(3).map(|s| s.as_str()).collect();

            let loader = PyExScrLoader::new()?;
            loader.execute_extension(module_name, &ext_args)?;
        }
        _ => {
            println!("Ember Dev Tool v0.1.0 - CatCode Labs");
            println!("Usage: ember <command>");
            println!("Commands:");
            println!("  build               Builds the target defined in .anvil");
            println!("  seal (alias: forge) Executes build and creates .sha256 seal sidecar");
            println!("  clean (alias: recede) Purges workspace targets defined in purge_paths");
            println!("  run-ext <module>    Executes a local exscr.ember-<module>.py extension");
        }
    }

    Ok(())
}