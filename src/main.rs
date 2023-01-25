use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Kahlia Hogg",
    about = "AWS S3 CLI in Rust",
    after_help = "Example: aws-cli"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Buckets {},
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let client = awscli::client().await.unwrap();
    match args.command {
        Some(Commands::Buckets {}) => {
            awscli::list_buckets(&client).await.unwrap();
        }
        None => println!("No command specified"),
    }
}
