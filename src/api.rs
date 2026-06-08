use reqwest::Client;
use std::fs;

pub struct FcloudClient {
    client: Client,
    base_url: String,
}

impl FcloudClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn get_list(&self, path: String) -> Result<Vec<String>, reqwest::Error> {
        let url = format!("{}/list", self.base_url);
        let files: Vec<String> = self.client.get(url)
            .header("path", path)
            .send()
            .await?
            .json()
            .await?;
        Ok(files)
    }

    pub async fn upload_file(&self, local_path: &str, target_name: &str) -> Result<u16, Box<dyn std::error::Error>> {
        let url = format!("{}/send", self.base_url);
        
        let file_bytes = fs::read(local_path)?;

        let res = self.client.post(url)
            .header("file_name", target_name)
            .body(file_bytes)
            .send()
            .await?;

        Ok(res.status().as_u16())
    }
}
