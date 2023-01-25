// AWS S3 Configuration
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Error};

// Create AWS S3 client
pub async fn client() -> Result<Client, Error> {
    let region_provider = RegionProviderChain::first_try(None)
        .or_default_provider()
        .or_else("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    Ok(client)
}

// List all buckets
pub async fn list_buckets(client: &Client) -> Result<(), Error> {
    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets().unwrap_or_default();
    let num_buckets = buckets.len();

    for bucket in buckets {
        println!("{}", bucket.name().unwrap_or_default());
    }

    println!();
    println!("Found {} buckets.", num_buckets);

    Ok(())
}

// Create new bucket
// pub async fn create_bucket(client: &Client, bucket_name: &str, region: &str) -> Result<(), Error> {
//     let constraint = BucketLocationConstraint::from(region);
//     let cfg = CreateBucketConfiguration::builder()
//         .location_constraint(constraint)
//         .build();
//     client
//         .create_bucket()
//         .create_bucket_configuration(cfg)
//         .bucket(bucket_name)
//         .send()
//         .await?;
//     println!("Creating bucket named: {bucket_name}");
//     Ok(())
// }
