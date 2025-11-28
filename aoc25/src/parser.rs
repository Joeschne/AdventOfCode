use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Day to process.
    #[arg(short, long)]
    pub day: usize,

    /// Part to process (1 or 2)
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
    pub part: u8,

    /// Whether part 1 and 2 have different input files
    #[arg(short, long, default_value_t = false)]
    pub separate_input_files: bool,
}
