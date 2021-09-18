use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct GetClient {
    url: String,
    api_key: String,
    user: String,
    password: String,
}

impl GetClient {
    pub fn builder() -> GetClientBuilder {
        GetClientBuilder::default()
    }
    pub async fn send(self) -> Result<Issues, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client
            .get(self.url)
            .header("X-Redmine-API-Key", self.api_key)
            .send()
            .await?;
        let result = response.text().await?;

        let issues: Issues = serde_json::from_str(&result)?;
        Ok(issues)
    }
}

#[derive(Debug, Default)]
pub struct GetClientBuilder {
    url: String,
    api_key: String,
    user: String,
    password: String,
}

impl GetClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.api_key = key.into();
        self
    }
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = user.into();
        self
    }
    pub fn password(mut self, password: String) -> Self {
        self.password = password;
        self
    }
    pub fn build(self) -> GetClient {
        GetClient {
            url: self.url,
            api_key: self.api_key,
            user: self.user,
            password: self.password,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Issues {
    issues: Vec<Issue>,
    total_count: i64,
    offset: i64,
    limit: i64,
}

#[derive(Debug, Deserialize)]
struct Issue {
    id: i64,
    project: ItemOne,
    tracker: ItemOne,
    status: ItemOne,
    priority: ItemOne,
    author: ItemOne,
    fixed_version: ItemOne,
    subject: String,
    description: String,
    start_date: String,
    due_date: Option<String>,
    done_ratio: i64,
    is_private: bool,
    estimated_hours: Option<String>,
    custom_fields: Vec<CustomField>,
    created_on: String,
    updated_on: String,
    closed_on: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ItemOne {
    id: i64,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum CustomField {
    ItemTwo {
        id: i64,
        name: String,
        value: String,
    },
    ItemThree {
        id: i64,
        name: String,
        multiple: bool,
        value: Vec<String>,
    },
}

#[tokio::test]
#[ignore]
async fn test_struct() {
    let url = "http://localhost:8000/projects/prj1/issues.json";
    let key = "d1b2c51db3fa1d6277b2e775447b05a58a1b5011";
    let client = GetClient::builder().url(url).key(key).build();
    let response = client.send().await;
    match response {
        Ok(rel) => println!("{:?}", rel),
        _ => println!("no"),
    }
}
