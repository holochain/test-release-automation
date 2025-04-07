use clap::{Parser, Subcommand};
use test_ra_core::{calculate_fibonacci, get_default_config, greet};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Greet a user
    Greet {
        /// Name of the person to greet
        #[arg(short, long)]
        name: String,
    },
    /// Show configuration
    Config,
    /// Generate Fibonacci sequence
    Fibonacci {
        /// Number of elements in the sequence
        #[arg(short, long, default_value_t = 10)]
        count: u32,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Greet { name } => {
            println!("{}", greet(name));
        }
        Commands::Config => {
            let config = get_default_config();
            println!("App Name: {}", config.app_name);
            println!("Version: {}", config.version);
            println!("Debug Mode: {}", config.debug_mode);
        }
        Commands::Fibonacci { count } => {
            let sequence = calculate_fibonacci(*count);
            println!("Fibonacci sequence ({} elements):", sequence.len());
            println!("Sequence: {:?}", sequence);
        }
    }
}
