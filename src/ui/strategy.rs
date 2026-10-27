use clap::Subcommand;

#[derive(Subcommand, Debug, Clone, Copy)]
pub enum Strategy {
    Bruteforce,
    Random {
        #[arg(short, long, default_value = "1")]
        samples: usize,
    },
    Swap, // TODO: Cuando se termine con la estrategia swap, esto podría cambiar.
    LocalSearch {
        #[arg(short, long, default_value = "1")]
        samples: usize,
    },
}
