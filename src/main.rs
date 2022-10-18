use std::process::exit;
use clap::{Parser, Args, Subcommand};

/// Proxy tcp connections to another listener
#[derive(Parser)]
#[command(version)]
#[command(about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// port for proxy server
    proxy_port: Option<String>,

    /// port for destination server
    server_port: Option<String>,

    /// print build information and exit
    #[arg(short, long)]
    build: bool,
}

fn main() {
    let args = Cli::parse();

    if args.build {
        buildInfo();
    }
}

fn buildInfo() {
    println!("Build Timestamp: {}", env!("VERGEN_BUILD_TIMESTAMP"));
    println!("git semver: {}", env!("VERGEN_GIT_SEMVER"));
    println!("git branch: {}", env!("VERGEN_GIT_BRANCH"));
    println!("git SHA SHORT: {}", env!("VERGEN_GIT_SHA_SHORT"));

    // let bi = json!({
    //     "buildTimestamp": env!("VERGEN_BUILD_TIMESTAMP"),
    //     "branch": env!("VERGEN_GIT_BRANCH"),
    //     "commit": env!("VERGEN_GIT_SHA_SHORT"),
    //     "version": env!("VERGEN_GIT_SEMVER")
    // });
    //
    // let bbi = BuildInfo{
    //     build_timestamp: env!("VERGEN_BUILD_TIMESTAMP").to_string(),
    //     branch: env!("VERGEN_GIT_BRANCH").to_string(),
    //     commit: env!("VERGEN_GIT_SHA_SHORT").to_string(),
    //     version: env!("VERGEN_GIT_SEMVER").to_string(),
    //     test: vec!["hello".to_string(), "hi there".to_string()]
    // };
    //
    // println!("{}", serde_json::to_string_pretty(&bbi).unwrap());
    // println!("{}", serde_yaml::to_string(&bbi).unwrap());
    // println!("{}", xmlserde::xml_serialize(&bbi));
}
