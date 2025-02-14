use crate::utils::default_home_dir;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    about = "gitrack",
    version = env!("CARGO_PKG_VERSION"),
    author = "Containerscrew info@containerscrew.com",
    about = "Inspect your untracked local Git files",
    arg_required_else_help = false
)]
pub struct Args {
    #[arg(
        short = 'p',
        long = "path",
        help = "Folder path you want to scan for git untracked files",
        default_value_t = default_home_dir(),
        required = false,
    )]
    pub path: String,

    #[arg(
        short = 'w',
        long = "workers",
        help = "Number of threads to use for scanning repositories",
        default_value_t = 5,
        value_parser(clap::value_parser!(u16).range(1..=10)),
        required = false
    )]
    pub workers: u16,

    #[arg(
        short = 'd',
        long = "diff",
        help = "Show differences between changed files",
        default_value_t = false,
        required = false
    )]
    pub diff: bool,

    #[arg(
        short = 'e',
        long = "exclude-dir",
        help = "Exclude directories to scan",
        value_delimiter = ' ',
        num_args = 1..,
        required = false
    )]
    pub exclude: Option<Vec<String>>,

    #[arg(
        short = 'u',
        long = "check-untracked",
        help = "Only show repositories with untracked files",
        default_value_t = false,
        required = false
    )]
    pub check_untracked: bool,

    #[arg(
        short = 'v',
        long = "verbose",
        help = "Print verbose output (untracked files)",
        default_value_t = false,
        required = false
    )]
    pub verbose: bool,
}
