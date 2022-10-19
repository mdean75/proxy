use std::process::exit;
use clap::{Parser, Args, Subcommand, ValueEnum};

extern crate serde_json;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

/// Proxy tcp connections to another listener
#[derive(Parser)]
#[command(version)]
#[command(about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// print build information and exit
    #[arg(short, long)]
    build: bool,

    /// output format (not available with all commands)
    #[arg(short, long, requires = "build")]
    output_format: Option<Mode>,
}

#[derive(Subcommand)]
enum Commands {
    Serve(Serve),
}

#[derive(Args)]
struct Serve {
    /// port for proxy server
    #[arg(short, long)]
    proxy_port: String,

    /// port for destination server
    #[arg(short, long)]
    server_port: String,
}

#[derive(Debug)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// JSON formatted output
    Json,
    /// YAML formatted output
    Yaml,
}

fn main() {
    let args = Cli::parse();

    if args.build {

        build_info(args.output_format);
        exit(0);
    }

    // match on the commands
    match &args.command {
        // args.command is of type Option, check for both some and none
        Some(serve) => {

            // we have something, now match on its type and handle it
            match &serve {
                Commands::Serve(s) => {
                    println!("server port: {}", s.server_port);
                    println!("proxy port: {}", s.proxy_port);
                }
            }
        }

        None => {
            println!("wtf!")
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
struct BuildInfo {
    build_timestamp: String,
    branch: String,
    commit: String,
    version: String,
}

fn build_info(format: Option<Mode>) {

    let bbi = BuildInfo{
        build_timestamp: env!("VERGEN_BUILD_TIMESTAMP").to_string(),
        branch: env!("VERGEN_GIT_BRANCH").to_string(),
        commit: env!("VERGEN_GIT_SHA_SHORT").to_string(),
        version: env!("VERGEN_GIT_SEMVER").to_string(),
    };

    if format.is_some() {
        match format.unwrap() {
            Mode::Json => {
                println!("{}", serde_json::to_string_pretty(&bbi).unwrap());
            }

            Mode::Yaml => {
                println!("{}", serde_yaml::to_string(&bbi).unwrap());
            }
        }
    } else {
        println!("Build Timestamp: {}", env!("VERGEN_BUILD_TIMESTAMP"));
        println!("git semver: {}", env!("VERGEN_GIT_SEMVER"));
        println!("git branch: {}", env!("VERGEN_GIT_BRANCH"));
        println!("git SHA SHORT: {}", env!("VERGEN_GIT_SHA_SHORT"));
    }
}
