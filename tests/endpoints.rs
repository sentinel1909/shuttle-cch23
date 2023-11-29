// tests/endpoints.rs

// tests for Challenge -1
mod endpoint_tests {

    #[test]
    fn test_root() {
        
        let response = reqwest::blocking::get("http://127.0.0.1:8000").unwrap();
        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
    }

    #[test]
    fn test_fake_error() {
        let response = reqwest::blocking::get("http://127.0.0.1:8000/-1/error").unwrap();
        assert!(response.status().is_server_error());
    }
}
