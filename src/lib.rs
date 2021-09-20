mod client;
pub use client::get_client::*;
pub use client::post_client::*;

#[cfg(test)]
mod tests {
    use super::GetClient;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn test_get_request() {
        let url = "http://localhost:8000";
        let key = "d1b2c51db3fa1d6277b2e775447b05a58a1b5011";
        let client = GetClient::builder().base_url(url).key(key).build();
        let response = client.get_project("prj1").await;
        assert!(response.is_ok());
    }
}
