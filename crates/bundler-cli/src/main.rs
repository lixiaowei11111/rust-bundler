use anyhow::Result;
use bundler_core::{Bundler, Config};
use clap::{Arg, Command};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::init();

    let matches = Command::new("rust-bundler")
        .version("0.1.0")
        .about("A Webpack-like bundler written in Rust")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .subcommand(Command::new("init-types").about("Generate TypeScript type definitions"))
        .arg(
            Arg::new("entry")
                .help("Entry point of the application")
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output directory")
                .default_value("dist"),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Path to config file (.json, .ts, .mts)")
                .value_name("FILE"),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("init-types", _)) => {
            #[cfg(feature = "generate-types")]
            {
                bundler_core::generate_typescript_definitions()?;
                return Ok(());
            }
            #[cfg(not(feature = "generate-types"))]
            {
                println!(
                    "Type generation not enabled. Please rebuild with --features generate-types"
                );
                return Ok(());
            }
        }
        _ => {
            // Original bundling logic
            let entry = matches.get_one::<String>("entry");
            if entry.is_none() {
                println!("Error: Entry point is required");
                return Ok(());
            }

            // ... rest of existing code ...
        }
    }

    let entry = matches.get_one::<String>("entry").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    let config_path = matches.get_one::<String>("config");

    let config = if let Some(config_path) = config_path {
        Config::from_file(config_path).await?
    } else {
        // Try to find default config files
        let default_configs = [
            "bundler.config.ts",
            "bundler.config.mts",
            "bundler.config.js",
            "bundler.config.json",
        ];

        let mut config = None;
        for default_config in &default_configs {
            if tokio::fs::metadata(default_config).await.is_ok() {
                config = Some(Config::from_file(default_config).await?);
                println!("📋 Using config file: {}", default_config);
                break;
            }
        }

        config.unwrap_or_else(|| Config::default().with_entry(entry).with_output_path(output))
    };

    let mut bundler = Bundler::new(config).await?;
    bundler.run().await?;

    println!("✅ Bundle completed successfully!");
    Ok(())
}
