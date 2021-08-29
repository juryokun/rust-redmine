// use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::env;
// use std::fs::File;
use std::io::{BufReader, Read};
use std::{collections::HashMap, sync::Mutex};
// use structopt::StructOpt;

#[tokio::main]

async fn main() {
    // let args = Cli::from_args();
    // match controll_subcommands(args.cmd, args.channel).await {
    //     Ok(exit_code) => exit_command(exit_code),
    //     Err(e) => {
    //         println!("{}", e);
    //         exit_command(ExitCode::ERROR)
    //     }
    // }
}
async fn get_request() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = "http://localhost:8080/projects/company/issues.json?query_id=1";
    // let url = "http://localhost:8080/projects/company/issues.json?issue_id=3";
    let response = client
        .get(url)
        .header(
            "X-Redmine-API-Key",
            "d44bf19219dbd92d882ef5a7063b71f01e8fc6b3",
        )
        .send()
        .await?;
    let result = response.text().await.unwrap();

    let issues: Issues = serde_json::from_str(&result).unwrap();
    // println!("{:?}", issues);
    for issue in issues.issues.iter() {
        println!("{}", issue.subject);
        println!("{}", issue.project.name);
        println!("{}", issue.author.name);
    }
    Ok(())
}

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

#[tokio::test]
async fn test_get_request() {
    let rel = get_request().await;
}

#[tokio::test]
async fn test_post_request() {
    let rel = post_request().await;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
