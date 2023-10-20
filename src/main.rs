use clap::Parser;
use evergreen::errors::Error;
use evergreen::clap::Cli;


fn main() -> Result<(), Error> {
    let args = Cli::parse();

    eprintln!("{:?}", args);
    Ok(())
}

