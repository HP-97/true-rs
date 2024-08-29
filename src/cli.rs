use clap::Parser;

const AFTER_HELP: &'static str = "\
NOTE: your shell may have its own version of true, which usually supersedes
the version described here.  Please refer to your shell's documentation
for details about the options it supports.
";

#[derive(Parser)]
#[command(version, verbatim_doc_comment, after_help=AFTER_HELP)]
/// Exit with a status code indicating success.
pub struct Cli {}

pub fn parse_args() -> Cli {
    let cli = Cli::parse();
    cli
}
