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
    Bucket {
        #[clap(short, long)]
        action: String,
    },
    Object {
        #[clap(short, long)]
        action: String,
        #[clap(short, long)]
        bucket: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let client = s3cli::client().await.unwrap();
    let region = s3cli::get_region().await.unwrap();
    // Match on subcommand
    match args.command {
        Some(Commands::Bucket { action }) => match action.as_str() {
            "list" => {
                s3cli::list_buckets(&client).await.unwrap();
            }
            "create" => {
                s3cli::create_bucket(&client, "test-bucket", &region)
                    .await
                    .unwrap();
            }
            _ => {
                println!("Invalid action");
            }
        },
        Some(Commands::Object { action, bucket }) => match action.as_str() {
            "list" => {
                s3cli::list_objects(&client, &bucket).await.unwrap();
            }
            _ => {
                println!("Invalid action");
            }
        },
        None => {
            println!("No subcommand was used");
        }
    }
}
