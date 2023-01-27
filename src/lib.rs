// AWS S3 Configuration
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::model::{BucketLocationConstraint, CreateBucketConfiguration};
use aws_sdk_s3::{Client, Error};

// Determine AWS region
pub async fn get_region() -> Result<String, Error> {
    let region_provider = RegionProviderChain::first_try(None)
        .or_default_provider()
        .or_else("us-east-1");
    let region_str: String = String::from(region_provider.region().await.unwrap().as_ref());
    Ok(region_str)
}

// Create AWS client
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
    println!("Found {} buckets.", num_buckets);
    println!();
    for bucket in buckets {
        println!("{}", bucket.name().unwrap_or_default());
    }

    Ok(())
}

// Create new bucket
pub async fn create_bucket(client: &Client, bucket_name: &str, region: &str) -> Result<(), Error> {
    let constraint = BucketLocationConstraint::from(region);
    print!("{:?}", constraint);
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();
    client
        .create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(bucket_name)
        .send()
        .await?;
    println!("Creating bucket named: {bucket_name} in region: {region}");
    Ok(())
}

// List objects in bucket
pub async fn list_objects(client: &Client, bucket: &str) -> Result<(), Error> {
    let resp = client.list_objects_v2().bucket(bucket).send().await?;
    let objects = resp.contents().unwrap_or_default();
    let num_objects = objects.len();

    println!("Found {num_objects} in bucket {bucket}");
    println!();
    for object in objects {
        println!("{}", object.key().unwrap_or_default());
    }

    Ok(())
}
