use clap::Parser;

#[derive(Debug, Parser)]
#[command(
author = "arsiac",
version = "0.0.1",
about = "Clear Java Project",
long_about = None
)]
pub struct Argument {
    #[arg(
    long,
    default_value_t = false,
    help = "Remove Maven 'target' directories"
    )]
    pub maven: bool,
    #[arg(long, default_value_t = false, help = "Remove IDEA project's files ")]
    pub idea: bool,
    #[arg(long, default_value_t = false, help = "Set log level to debug")]
    pub debug: bool,
    #[arg()]
    pub path: String,
}
