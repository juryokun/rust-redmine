pub mod get_client;
pub mod post_client;
pub use get_client::*;
pub use post_client::*;

#[cfg(test)]
mod tests {
    use super::GetClient;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn test_get_request() {
        let url = "http://localhost:8000/projects/prj1/issues.json";
        let key = "d1b2c51db3fa1d6277b2e775447b05a58a1b5011";
        let client = GetClient::builder().url(url).key(key).build();
        let response = client.send().await;
        assert!(response.is_ok());
    }
}
