use crate::bucket::*;
use crate::command::Command;
use crate::request::RequestImpl;

impl Bucket {
    /// Head object from S3.
    ///
    /// # Example:
    ///
    /// ```no_run
    /// use s3::bucket::Bucket;
    /// use s3::creds::Credentials;
    /// use anyhow::Result;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    ///
    /// let bucket_name = "rust-s3-test";
    /// let region = "us-east-1".parse()?;
    /// let credentials = Credentials::default()?;
    /// let bucket = Bucket::new(bucket_name, region, credentials)?;
    ///
    /// let (head_object_result, code) = bucket.head_object("/test.png").await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn head_object<S: AsRef<str>>(
        &self,
        path: S,
    ) -> Result<(HeadObjectResult, u16), S3Error> {
        let command = Command::HeadObject;
        let request = RequestImpl::new(self, path.as_ref(), command)?;
        let (headers, status) = request.response_header().await?;
        let header_object = HeadObjectResult::from(&headers);
        Ok((header_object, status))
    }

    /// Check if an object exists in S3.
    /// Uses `HeadObject` under the hood.
    ///
    /// # Example:
    ///
    /// ```no_run
    /// use s3::bucket::Bucket;
    /// use s3::creds::Credentials;
    /// use anyhow::Result;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    ///
    /// let bucket_name = "rust-s3-test";
    /// let region = "us-east-1".parse()?;
    /// let credentials = Credentials::default()?;
    /// let bucket = Bucket::new(bucket_name, region, credentials)?;
    ///
    /// let file_exists = bucket.object_exists("/test.png").await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn object_exists(&self, path: impl AsRef<str>) -> Result<bool, S3Error> {
        let command = Command::HeadObject;
        let request = RequestImpl::new(self, path.as_ref(), command)?;
        let response = request.response().await?;
        match response.status().as_u16() {
            // I'd prefer 200..<300, but that's not implemented yet, and might never be.
            200..=299 => Ok(true),
            404 => Ok(false),
            _ => Err(S3Error::HttpFail),
        }
    }
}
