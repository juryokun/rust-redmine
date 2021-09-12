use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct PostClient {
    url: String,
    api_key: String,
    user: String,
    password: String,
    data: NewIssue,
}

impl PostClient {
    pub fn builder() -> PostClientBuilder {
        PostClientBuilder::default()
    }
    pub async fn send(self) {
        let client = reqwest::Client::new();

        let response = client
            .post(self.url)
            .json(&self.data)
            .header("X-Redmine-API-Key", self.api_key)
            .send()
            .await
            .unwrap();
        let result = response.text().await;
        println!("{:?}", result);
    }
}

#[derive(Debug, Default)]
pub struct PostClientBuilder {
    url: String,
    api_key: String,
    user: String,
    password: String,
    data: NewIssue,
}

impl PostClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }
    pub fn key(mut self, key: String) -> Self {
        self.api_key = key;
        self
    }
    pub fn user(mut self, user: String) -> Self {
        self.user = user;
        self
    }
    pub fn password(mut self, password: String) -> Self {
        self.password = password;
        self
    }
    pub fn build(self) -> PostClient {
        PostClient {
            url: self.url,
            api_key: self.api_key,
            user: self.user,
            password: self.password,
            data: self.data,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct NewIssue {
    issue: NewIssueContent,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct NewIssueContent {
    project_id: i64,
    tracker_id: i64,
    status_id: i64,
    priority_id: i64,
    subject: String,
    description: String,
    fixed_version_id: i64,
    is_private: bool,
    estimated_hours: i64,
    custom_fields: Vec<NewCustomField>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum NewCustomField {
    NewItemTwo { id: i64, value: String },
    NewItemThree { id: i64, value: Vec<String> },
}

impl NewIssue {
    fn new() -> Self {
        Self {
            issue: NewIssueContent::new(),
        }
    }
}

impl NewIssueContent {
    fn new() -> Self {
        Self {
            project_id: 1,
            tracker_id: 1,
            status_id: 1,
            priority_id: 1,
            subject: "postテスト".to_string(),
            description: "説明分".to_string(),
            fixed_version_id: 1,
            is_private: false,
            estimated_hours: 4,
            custom_fields: vec![NewCustomField::NewItemTwo {
                id: 1,
                value: "あいう".to_string(),
            }],
        }
    }
}
