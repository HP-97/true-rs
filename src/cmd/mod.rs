use crate::cli::Cli;

/// Returns true - no matter what
pub fn run(_args: Cli) -> anyhow::Result<bool> {
    Ok(true)
}
