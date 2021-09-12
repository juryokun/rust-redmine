use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
struct Client {
    url: String,
    api_key: String,
    user: String,
    password: String,
}

#[derive(Debug, Default)]
struct GetClient {
    client: Client,
}

impl GetClient {
    fn new() -> Self {
        Self::default()
    }
    fn url(self, url: String) -> Self {
        Self {
            client: Client {
                url: url,
                api_key: self.client.api_key,
                user: self.client.user,
                password: self.client.password,
            },
        }
    }
    fn key(self, key: String) -> Self {
        Self {
            client: Client {
                url: self.client.url,
                api_key: key,
                user: self.client.user,
                password: self.client.password,
            },
        }
    }
    async fn send(&self) {
        let client = reqwest::Client::new();
        let response = client
            .get(&self.client.url)
            .header("X-Redmine-API-Key", &self.client.api_key)
            .send()
            .await
            .unwrap();
        println!("{:?}", response);
        let result = response.text().await.unwrap();

        let issues: Issues = serde_json::from_str(&result).unwrap();
        // println!("{:?}", issues);
        for issue in issues.issues.iter() {
            println!("{}", issue.subject);
            println!("{}", issue.project.name);
            println!("{}", issue.author.name);
        }
    }
}

#[derive(Debug, Deserialize)]
struct Issues {
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
async fn test_struct() {
    let url = "http://localhost:8080/projects/alpha/issues.json".to_string();
    let key = "f111fc80e00156d8fe0ac520a2ea7b21a5d984be".to_string();
    let client = GetClient::new();
    let result = client.url(url).key(key).send().await;
}
