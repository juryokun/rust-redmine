extern crate rust_redmine;
use rust_redmine::*;

#[tokio::test]
#[ignore]
async fn get_from_redmine() {
    let url = "http://localhost:8000/";
    // let base_url = "http://localhost:8000/projects/prj1/issues.json";
    let key = "d1b2c51db3fa1d6277b2e775447b05a58a1b5011";
    let client = GetClient::builder().base_url(url).key(key).build();
    let response = client.get_project("prj1").await;
    assert!(response.is_ok());
    println!("{:?}", response);
}

#[tokio::test]
#[ignore]
async fn get_issue() {
    let url = "http://localhost:8000";
    let key = "d1b2c51db3fa1d6277b2e775447b05a58a1b5011";
    let client = GetClient::builder().base_url(url).key(key).build();
    let response = client.get_issue(1).await;
    assert!(response.is_ok());
    println!("{:?}", response);
}

#[tokio::test]
async fn post_newissue() {
    let mut issue = NewIssue::new();
    issue.set_description("description");
    issue.set_project_id(1);
    issue.set_priority_id(1);
    issue.set_status_id(1);
    issue.set_tracker_id(1);
    issue.set_assigned_to_id(1);
    issue.set_custom_field_value(2, "あたい１");
    issue.set_custom_field_multiple_value(1, vec!["1", "3"]);
    issue.set_subject("pot_request");
    issue.set_fixed_version_id(1);

    let url = "https://httpbin.org/anything";
    // let url = "http://localhost:8000/projects/prj1/issues.json";
    let key = "d1b2c51db3fa1d6277b2e775447b05a58a1b5011";
    let client = PostClient::builder().url(url).key(key).data(issue).build();
    let response = client.send().await;
    assert!(response.is_ok());
}
