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
    Buckets {
    },
    Create {
        #[clap(short, long)]
        bucket: String,
        // #[clap(short, long)]
        // region: String,
    },
    Objects {
        #[clap(short, long)]
        bucket: String,
    },
    Upload {
        #[clap(short, long)]
        bucket: String,
        #[clap(short, long)]
        filepath: String
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let client = s3cli::client().await.unwrap();
    // Match on subcommand
    match args.command {
        Some(Commands::Buckets { }) => {
            s3cli::list_buckets(&client).await.unwrap();
        }
        Some(Commands::Create { bucket }) => {
            let bucket_region = s3cli::bucket_region().await.unwrap();
            s3cli::create_bucket(&client, &bucket, &bucket_region)
                .await
                .unwrap();
        }
        Some(Commands::Objects { bucket }) => {
            s3cli::list_objects(&client, &bucket).await.unwrap();
            }
        Some(Commands::Upload { bucket, filepath }) => {
            s3cli::upload_object(&client, &bucket, &filepath).await.unwrap();
            }
        None => {
            println!("No subcommand was used");
        }
    }
}
