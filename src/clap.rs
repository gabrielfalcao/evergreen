// use crate::coreio::ensure_dir_exists;
// use crate::errors::Error;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Note(NoteParams),
}
#[derive(Args, Debug)]
pub struct NoteParams {
    #[command(subcommand)]
    pub command: NoteCommands,
}

#[derive(Debug, Subcommand)]
pub enum NoteCommands {
    #[command(about = "add setting")]
    Add(NoteAddParams),

    #[command(about = "get setting")]
    Get(NoteGetParams),

    #[command(about = "list note")]
    List(NoteListParams),

    #[command(about = "delete setting")]
    Delete(NoteDeleteParams),
}

#[derive(Args, Debug)]
pub struct NoteAddParams {
    pub id: String,
}
#[derive(Args, Debug)]
pub struct NoteGetParams {
    pub id: String,
}
#[derive(Args, Debug)]
pub struct NoteListParams {
    #[clap(long = "json")]
    pub json: bool,
}
#[derive(Args, Debug)]
pub struct NoteDeleteParams {
    pub id: String,
}
