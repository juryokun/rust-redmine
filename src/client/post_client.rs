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
                .danger_accept_invalid_certs(self.insecure)
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
    pub fn insecure(mut self, is_insecure: bool) -> Self {
        self.insecure = is_insecure;
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
