use super::config::PocketBaseConfig;
use super::models::{ListResponse, PBTask};
use reqwest::blocking::Client;

pub struct PocketBaseClient {
    client: Client,
    base_url: String,
}

impl PocketBaseClient {
    pub fn new(config: &PocketBaseConfig) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
            base_url: config.server_url.trim_end_matches('/').to_string(),
        }
    }

    pub fn health_check(&self) -> bool {
        self.client
            .get(format!("{}/api/health", self.base_url))
            .send()
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    // Tasks API
    pub fn list_tasks(&self) -> anyhow::Result<Vec<PBTask>> {
        let url = format!(
            "{}/api/collections/tasks/records?perPage=500",
            self.base_url
        );
        let response: ListResponse<PBTask> =
            self.client.get(&url).send()?.error_for_status()?.json()?;
        Ok(response.items)
    }

    pub fn create_task(&self, task: &PBTask) -> anyhow::Result<PBTask> {
        let url = format!("{}/api/collections/tasks/records", self.base_url);
        let response: PBTask = self
            .client
            .post(&url)
            .json(task)
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response)
    }

    pub fn update_task(&self, remote_id: &str, task: &PBTask) -> anyhow::Result<PBTask> {
        let url = format!(
            "{}/api/collections/tasks/records/{}",
            self.base_url, remote_id
        );
        let response: PBTask = self
            .client
            .patch(&url)
            .json(task)
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response)
    }

    pub fn delete_task(&self, remote_id: &str) -> anyhow::Result<()> {
        let url = format!(
            "{}/api/collections/tasks/records/{}",
            self.base_url, remote_id
        );
        self.client.delete(&url).send()?.error_for_status()?;
        Ok(())
    }
}
