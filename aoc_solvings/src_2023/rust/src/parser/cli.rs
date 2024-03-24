use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    // which days to run and time
    #[arg(value_parser = clap::value_parser!(u16).range(1..=25))]
    pub days: Vec<u16>,
    // adjust logging level
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
