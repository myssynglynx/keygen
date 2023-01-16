use clap::{Parser};
use keygen::keygen;

/// CLI Keygen Tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Length of key to generate
    #[arg(short, long, default_value_t = 8)]
    length: usize,

    /// Format of key to generate (uppercase, lowercase, numeric, alphanumeric, base64url)
    #[arg(short, long, default_value = "numeric")]
    format: String,

    /// Generation seed
    #[arg(short, long, default_value = "hello world")]
    seed: String,

    /// Deterministically generate key [defaut: false]
    #[arg(short, long, default_value = "false")]
    deterministic: bool,

    /// Number of keys to generate
    #[arg(short, long, default_value_t = 1)]
    takes: usize
}

fn main() {
    let args = Args::parse();

    let config = keygen::Config::new(args.length, args.format, args.seed, args.takes);

    let keys = if args.deterministic == true {
        println!("Deterministically generating {} random keys", args.takes);
        keygen::generate_deterministic_random_key(&config)
    } else {
        println!("Generating {} random keys", args.takes);
        keygen::generate_random_key(&config)
    };

    for key in keys {
        println!("{}", &key);
    }
}
