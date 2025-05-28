use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version, about,
    long_about = None,
)]
pub struct Args {
    /// Always pick the first EFI result if there are multiple matches
    #[arg(short, long)]
    pub force_first: bool,

    /// Allow picking inactive boot records
    #[arg(short = 'i', long)]
    pub allow_inactive: bool,

    /// Restart the PC after setting the variable
    #[arg(short, long)]
    pub restart: bool,

    /// Keyword (full-text search) used to search for the boot record to select
    pub search_keyword: Option<String>,
}