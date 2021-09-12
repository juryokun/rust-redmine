use serde::{Deserialize, Serialize};

async fn post_request() -> Result<(), Box<dyn std::error::Error>> {
    let issue = NewIssue::new();
    // let json = serde_json::to_string_pretty(&issue).unwrap();
    // println!("{}", json);

    // let client = reqwest::Client::new();
    let client = reqwest::Client::builder().build()?;
    // let url = "http://localhost:8080/projects/company/issues.json?query_id=1";
    let url = "http://localhost:8080/projects/company/issues.json";

    let response = client
        .post(url)
        // .body(json)
        .json(&issue)
        .header(
            "X-Redmine-API-Key",
            "d44bf19219dbd92d882ef5a7063b71f01e8fc6b3",
        )
        // .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await?;
    let result = response.text().await;
    println!("{:?}", result);
    // println!("{:?}", response);

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct NewIssue {
    issue: NewIssueContent,
}

#[derive(Debug, Deserialize, Serialize)]
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
