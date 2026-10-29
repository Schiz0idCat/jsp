use clap::Subcommand;

#[derive(Subcommand, Debug, Clone, Copy)]
pub enum Strategy {
    Bruteforce,
    Random {
        #[arg(short, long, default_value = "1")]
        samples: usize,
    },
    Swap,
    LocalSearch {
        #[arg(short, long, default_value = "1")]
        samples: usize,
    },
}
