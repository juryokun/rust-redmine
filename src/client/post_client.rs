use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct PostClient {
    url: String,
    api_key: String,
    // user: String,
    // password: String,
    data: NewIssue,
    cert_file_path: String,
    insecure: bool,
}

impl PostClient {
    pub fn builder() -> PostClientBuilder {
        PostClientBuilder::default()
    }
    pub async fn send(self) -> Result<String, Box<dyn std::error::Error>> {
        let client = match self.cert_file_path.is_empty() {
            true => reqwest::Client::builder()
                .danger_accept_invalid_certs(self.insecure)
                .build()?,
            false => reqwest::Client::builder()
                .add_root_certificate(self.get_cert()?)
                .build()?,
        };

        let response = client
            .post(self.url)
            .json(&self.data)
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
pub struct PostClientBuilder {
    url: String,
    api_key: String,
    // user: String,
    // password: String,
    data: NewIssue,
    cert_file_path: String,
    insecure: bool,
}

impl PostClientBuilder {
    pub fn new() -> Self {
        Self {
            insecure: false,
            ..Self::default()
        }
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.api_key = key.into();
        self
    }
    pub fn data(mut self, data: NewIssue) -> Self {
        self.data = data;
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
    pub fn build(self) -> PostClient {
        PostClient {
            url: self.url,
            api_key: self.api_key,
            // user: self.user,
            // password: self.password,
            data: self.data,
            cert_file_path: self.cert_file_path,
            insecure: self.insecure,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct NewIssue {
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
    assigned_to_id: i64,
    is_private: bool,
    estimated_hours: i64,
    custom_fields: Vec<NewCustomField>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum NewCustomField {
    NewItemTwo { id: i64, value: String },
    NewItemMultiple { id: i64, value: Vec<String> },
}

impl NewIssue {
    pub fn new() -> Self {
        Self::default()
        // Self {
        //     issue: NewIssueContent::new(),
        // }
    }
    pub fn set_project_id(&mut self, project_id: i64) {
        self.issue.project_id = project_id;
    }
    pub fn set_tracker_id(&mut self, tracker_id: i64) {
        self.issue.tracker_id = tracker_id;
    }
    pub fn set_status_id(&mut self, status_id: i64) {
        self.issue.status_id = status_id;
    }
    pub fn set_priority_id(&mut self, priority_id: i64) {
        self.issue.priority_id = priority_id;
    }
    pub fn set_subject(&mut self, subject: impl Into<String>) {
        self.issue.subject = subject.into();
    }
    pub fn set_description(&mut self, description: impl Into<String>) {
        self.issue.description = description.into();
    }
    pub fn set_assigned_to_id(&mut self, assigned_to_id: i64) {
        self.issue.assigned_to_id = assigned_to_id;
    }
    pub fn set_fixed_version_id(&mut self, fixed_version_id: i64) {
        self.issue.fixed_version_id = fixed_version_id;
    }
    pub fn set_is_private(&mut self, is_private: bool) {
        self.issue.is_private = is_private;
    }
    pub fn set_estimated_hours(&mut self, estimated_hours: i64) {
        self.issue.estimated_hours = estimated_hours;
    }
    pub fn set_custom_field_value(&mut self, field_id: i64, field_value: impl Into<String>) {
        let custom_field = NewCustomField::NewItemTwo {
            id: field_id,
            value: field_value.into(),
        };
        self.update_custom_field(custom_field);
    }
    pub fn set_custom_field_multiple_value(
        &mut self,
        field_id: i64,
        field_values: Vec<impl Into<String>>,
    ) {
        let custom_field = NewCustomField::NewItemMultiple {
            id: field_id,
            value: field_values.into_iter().map(|x| x.into()).collect(),
        };
        self.update_custom_field(custom_field);
    }
    fn update_custom_field(&mut self, custom_field: NewCustomField) {
        let field_id = NewIssue::extract_custom_field_id(&custom_field);

        let mut target_id: isize = -1;
        for (i, field) in self.issue.custom_fields.iter().enumerate() {
            let id = NewIssue::extract_custom_field_id(field);
            if id == field_id {
                target_id = i as isize;
                break;
            }
        }
        if target_id != -1 {
            self.issue.custom_fields[target_id as usize] = custom_field;
        } else {
            self.issue.custom_fields.push(custom_field);
        }
    }
    fn extract_custom_field_id(custom_field: &NewCustomField) -> i64 {
        match custom_field {
            NewCustomField::NewItemTwo { id, .. } => *id,
            NewCustomField::NewItemMultiple { id, .. } => *id,
        }
    }
}

impl NewIssueContent {
    fn new() -> Self {
        Self::default()
        //     Self {
        //         project_id: 1,
        //         tracker_id: 1,
        //         status_id: 1,
        //         priority_id: 1,
        //         subject: "postテスト".to_string(),
        //         description: "説明分".to_string(),
        //         fixed_version_id: 1,
        //         is_private: false,
        //         estimated_hours: 4,
        //         custom_fields: vec![
        //             NewCustomField::NewItemTwo {
        //                 id: 1,
        //                 value: "あいう".to_string(),
        //             },
        //             NewCustomField::NewItemMultiple {
        //                 id: 2,
        //                 value: vec!["abc".to_string(), "def".to_string()],
        //             },
        //         ],
        //     }
    }
}

#[tokio::test]
async fn test_newissue() {
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
    println!("{:?}", issue);

    // let url = "https://httpbin.org/anything";
    let url = "http://localhost:8000/projects/prj1/issues.json";
    let key = "d1b2c51db3fa1d6277b2e775447b05a58a1b5011";
    let client = PostClient::builder().url(url).key(key).data(issue).build();
    let response = client.send().await;

    println!("{:?}", response);

    // issue.set_custom_field_value(1, "modify");
    // for field in issue.issue.custom_fields.iter() {
    //     match field {
    //         NewCustomField::NewItemTwo { id, value } => println!("{}", value),
    //         NewCustomField::NewItemMultiple { id, value } => println!("{:?}", value),
    //     }
    //     println!("{:?}", field);
    // }
}
