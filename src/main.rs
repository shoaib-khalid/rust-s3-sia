use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use std::str::FromStr;
//use s3::error::S3Error;
use s3::BucketConfiguration;
//use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials from environment variables
    //let access_key =
    //    env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID is set and a valid String");
    //let secret_key =
    //    env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY is set and a valid String");

    let access_key: &str = "AKIAVU7KJX4QIEYKKK4C";
    let secret_key: &str = "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";
    println!("Access Key is: {}", access_key); // Access Key is: AKIAVU7KJX4QAMXXQ7OC
    println!("Secret Key is: {}", secret_key); // Secret Key is: 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

    // AWS S3 credentials
    //let credentials = Credentials::from_profile(Some("default"))?;
    let credentials = Credentials::new(Some(access_key), Some(secret_key), None, None, None).unwrap();

    // Specify the S3 Region
    let region = Region::from_str("us-east-1")?;

    // Create a new bucket with the given name
    let bucket_name = "my4745-bt-154-091234";
    let _bucket = Bucket::new(bucket_name, region.clone(), credentials.clone())?;

    // Create the bucket
    match Bucket::create(
        bucket_name,
        region,
        credentials,
        BucketConfiguration::default(),
    )
    .await
    {
        Ok(_) => println!("Bucket created successfully."),
        Err(e) => eprintln!("Failed to create bucket: {}", e),
    }

    Ok(())
}
