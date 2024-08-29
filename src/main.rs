use std::io::Write;
mod cli;
mod cmd;

fn main() -> anyhow::Result<()> {
    // Initialise logging

    // Parse arguments
    let args = cli::parse_args();

    if let Err(err) = cmd::run(args) {
        writeln!(&mut std::io::stderr(), "{:?}", err).unwrap();
        std::process::exit(1);
    }

    Ok(())
}
