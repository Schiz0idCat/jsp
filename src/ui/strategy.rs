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
    SimulatedAnnealing {
        #[arg(short = 'i', long, default_value = "1000.0")]
        t_initial: f64,

        #[arg(short = 'm', long, default_value = "0.001")]
        t_min: f64,

        #[arg(short, long, default_value = "0.9995")]
        alpha: f64,
    },
}
