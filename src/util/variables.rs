use crate::util::result::Error;

use std::env;
use s3::{Region, creds::Credentials};

lazy_static! {
    // Application Settings
    pub static ref HOST: String =
        env::var("AUTUMN_HOST").expect("Missing AUTUMN_HOST environment variable.");
    pub static ref MONGO_URI: String =
        env::var("AUTUMN_MONGO_URI").expect("Missing AUTUMN_MONGO_URI environment variable.");
    pub static ref CORS_ALLOWED_ORIGIN: String =
        env::var("AUTUMN_CORS_ALLOWED_ORIGIN").expect("Missing AUTUMN_CORS_ALLOWED_ORIGIN environment variable.");
    pub static ref FILE_SIZE_LIMIT: usize =
        env::var("AUTUMN_FILE_SIZE_LIMIT").expect("Missing AUTUMN_FILE_SIZE_LIMIT environment variable.").parse().unwrap();

    // Storage Settings
    pub static ref LOCAL_STORAGE_PATH: String =
        env::var("AUTUMN_LOCAL_STORAGE_PATH").unwrap_or_else(|_| "./files".to_string());
    pub static ref S3_REGION: Region = Region::Custom {
        region: env::var("AUTUMN_S3_REGION").unwrap_or_else(|_| "".to_string()),
        endpoint: env::var("AUTUMN_S3_ENDPOINT").unwrap_or_else(|_| "".to_string())
    };
    pub static ref S3_CREDENTIALS: Credentials = Credentials::default().unwrap();
    
    // Application Flags
    pub static ref USE_S3: bool = env::var("AUTUMN_S3_REGION").is_ok() && env::var("AUTUMN_S3_ENDPOINT").is_ok();
}

pub fn get_s3_bucket() -> Result<s3::Bucket, Error> {
    s3::Bucket::new_with_path_style(&"file-uploads", S3_REGION.clone(), S3_CREDENTIALS.clone())
        .map_err(|_| Error::LabelMe)
}
