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
async fn test_get_request() {
    let rel = get_request().await;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
