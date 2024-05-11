use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(about)]
/// Mileage Mapper: A commandline application that maps your milage!

pub struct MilageCalculatorCLI {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Linear: Incrementally add steps specified amounts of time.
    Lin(LinearValues),

    /// Oscilator: Randomized linear segments
    Osc(OscilationValues),
}

#[derive(Debug, Args)]
pub struct OscilationValues {
    /// The amount to start with, e.g., 100,000.
    #[arg(short, long)]
    pub start_amount: u64,

    /// The amount of miles, e.g., 6.
    #[arg(short, long)]
    pub increment: u64,

    #[arg(short = 'L', long)]
    /// The lower bound offset (forms range with higher bound). Default: 7
    pub lo_offset: Option<u8>, 

    #[arg(short = 'H', long)]
    /// The higher bound offset (forms range with lower bound). Default: 12
    pub hi_offset: Option<u8>, 

    #[arg(short, long, num_args = 1..)] // the num_args = 1.. allows multiple arguments to be passed to Vec.
    /// The pattern (days) that represents the week; e.g., 2 3 2 1.
    pub pattern: Vec<u16>,
}

#[derive(Debug, Args)]
pub struct LinearValues {
    /// The amount to start with, e.g., 100,000.
    #[arg(short, long)]
    pub start_amount: u64,

    /// The amount of miles, e.g., 6.
    #[arg(short, long)]
    pub increment: u64,

    /// The amount of days to calculate mileage for.
    #[arg(short, long)]
    pub amount: u16,
}

pub fn parse_args() -> MilageCalculatorCLI {
    MilageCalculatorCLI::parse()
}
