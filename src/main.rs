use std::{
    process::{Stdio},
};
use clap::{
    Parser,
    arg,
    command,
};

#[derive(Clone, Debug, Parser)]
pub struct Args {
    #[arg(short, long)]
    provider: String,

    #[arg(short, long)]
    account: String,

    #[arg(short, long)]
    repository: String,
}

pub struct Installer {
    args: Args,
}

fn main() {
    let args = Args::parse();
    let provider = args.provider;
    let account = args.account;
    let repository = args.repository;
    
    let xd = std::process::Command::new("git")
        .arg("--help")
        .spawn()
        .expect("Failed to execute process");

}
