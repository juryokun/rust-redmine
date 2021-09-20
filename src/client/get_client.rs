use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone)]
pub struct GetClient {
    base_url: String,
    api_key: String,
    // user: String,
    // password: String,
    cert_file_path: String,
    insecure: bool,
}

impl GetClient {
    pub fn builder() -> GetClientBuilder {
        GetClientBuilder::default()
    }
    pub async fn get_project(
        self,
        project: impl Into<String>,
    ) -> Result<Issues, Box<dyn std::error::Error>> {
        let add_url = "/projects/".to_string() + &project.into() + "/issues.json";
        let request_url = self.base_url.trim_end_matches("/").to_string() + &add_url;
        let result = self.send(request_url).await?;

        let issues: Issues = serde_json::from_str(&result)?;
        Ok(issues)
    }
    pub async fn get_issue(self, issue_id: i64) -> Result<Issue, Box<dyn std::error::Error>> {
        let add_url = "/issues/".to_string() + &issue_id.to_string() + ".json";
        let request_url = self.base_url.trim_end_matches("/").to_string() + &add_url;
        let result = self.send(request_url).await?;

        let issue: Issue = serde_json::from_str(&result)?;
        Ok(issue)
    }
    pub async fn get_query(
        self,
        project: impl Into<String>,
        query_id: i64,
    ) -> Result<Issues, Box<dyn std::error::Error>> {
        let add_url =
            "/".to_string() + &project.into() + "/issues.json?query_id=" + &query_id.to_string();
        let request_url = self.base_url.trim_end_matches("/").to_string() + &add_url;
        let result = self.send(request_url).await?;

        let issues: Issues = serde_json::from_str(&result)?;
        Ok(issues)
    }
    async fn send(self, url: String) -> Result<String, Box<dyn std::error::Error>> {
        let client = match self.cert_file_path.is_empty() {
            true => reqwest::Client::builder()
                .danger_accept_invalid_certs(self.insecure)
                .build()?,
            false => reqwest::Client::builder()
                .add_root_certificate(self.get_cert()?)
                .build()?,
        };

        let response = client
            .get(url)
            .header("X-Redmine-API-Key", self.api_key)
            .send()
            .await?;
        let result = response.text().await?;
        Ok(result)
    }
    fn get_cert(&self) -> Result<reqwest::Certificate, Box<dyn std::error::Error>> {
        use std::io::Read;
        let mut buf = Vec::new();
        std::fs::File::open(self.cert_file_path.clone())?.read_to_end(&mut buf)?;
        let cert = reqwest::Certificate::from_der(&buf)?;
        Ok(cert)
    }
}

#[derive(Debug, Default)]
pub struct GetClientBuilder {
    base_url: String,
    api_key: String,
    // user: String,
    // password: String,
    cert_file_path: String,
    insecure: bool,
}

impl GetClientBuilder {
    pub fn new() -> Self {
        Self {
            insecure: false,
            ..Self::default()
        }
    }
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.api_key = key.into();
        self
    }
    // pub fn user(mut self, user: impl Into<String>) -> Self {
    //     self.user = user.into();
    //     self
    // }
    // pub fn password(mut self, password: impl Into<String>) -> Self {
    //     self.password = password.into();
    //     self
    // }
    pub fn cert_file_path(mut self, cert_file_path: impl Into<String>) -> Self {
        self.cert_file_path = cert_file_path.into();
        self
    }
    pub fn insecure(mut self, insecure: bool) -> Self {
        self.insecure = insecure;
        self
    }
    pub fn build(self) -> GetClient {
        GetClient {
            base_url: self.base_url,
            api_key: self.api_key,
            // user: self.user,
            // password: self.password,
            cert_file_path: self.cert_file_path,
            insecure: self.insecure,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Issues {
    issues: Vec<IssueContent>,
    total_count: i64,
    offset: i64,
    limit: i64,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    issue: IssueContent,
}

#[derive(Debug, Deserialize)]
struct IssueContent {
    id: i64,
    project: ItemOne,
    tracker: ItemOne,
    status: ItemOne,
    priority: ItemOne,
    author: ItemOne,
    fixed_version: Option<ItemOne>,
    assigned_to: Option<ItemOne>,
    subject: String,
    description: String,
    start_date: String,
    due_date: Option<String>,
    done_ratio: i64,
    is_private: bool,
    estimated_hours: Option<f64>,
    custom_fields: Option<Vec<CustomField>>,
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
    ItemMultiple {
        id: i64,
        name: String,
        multiple: bool,
        value: Vec<String>,
    },
}
