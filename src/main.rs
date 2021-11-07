use std::process;

use argh::FromArgs;
use confy;
use toml;

pub static DEFAULT_PATH: &str = "./nvhosts.toml";

/// Generate nginx vhosts from a configuration file
#[derive(FromArgs)]
struct Args {
    /// path to config file to use; defaults to ho.toml
    #[argh(option, short = 'c', default = "DEFAULT_PATH.to_string()")]
    config: String,

    /// show an example config
    #[argh(switch)]
    example: bool,

    /// show the version
    #[argh(switch, short = 'V')]
    version: bool,
}

fn main() {
    let args: Args = argh::from_env();

    if args.version {
        eprintln!(std::env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    if args.example {
        let config = nvhosts::Config::example();
        let example: String = toml::to_string_pretty(&config).unwrap_or_else(|err| {
            eprintln!("failed to print an example file {}: {}", args.config, err);
            process::exit(1);
        });
        print!("{}", example);
        process::exit(0);
    }

    let cfg: nvhosts::Config = confy::load_path(&args.config).unwrap_or_else(|err| {
        eprintln!("failed to load file {}: {}", args.config, err);
        process::exit(1);
    });

    nvhosts::generate(&cfg).unwrap_or_else(|err| {
        eprintln!("failed to run: {}", err);
        process::exit(1);
    });
}
