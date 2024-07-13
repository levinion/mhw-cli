use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    op: Op,
}

#[derive(Subcommand)]
pub enum Op {
    /// set exe path
    Init { path: String },
    /// list current mods
    List,
    /// remove a mod
    Remove { index: usize },
    /// add a mod
    Add { path: String },
    /// enable a mod
    Enable { index: usize },
    /// disable a mod
    Disable { index: usize },
    /// get a mod's status
    Status { index: usize },
}
