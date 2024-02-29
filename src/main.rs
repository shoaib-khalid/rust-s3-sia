use std::str::FromStr;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
//use s3::error::S3Error;
use s3::BucketConfiguration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // AWS S3 credentials
    let credentials = Credentials::from_profile(Some("default"))?;

    // Specify the S3 Region
    let region = Region::from_str("ap-southeast-1")?;

    // Create a new bucket with the given name
    let bucket_name = "my4745-bt-154-098jj";
    let _bucket = Bucket::new(bucket_name, region.clone(), credentials.clone())?;

    // Create the bucket
    match Bucket::create(bucket_name, region, credentials, BucketConfiguration::default()).await {
        Ok(_) => println!("Bucket created successfully."),
        Err(e) => eprintln!("Failed to create bucket: {}", e),
    }


    Ok(())
}
