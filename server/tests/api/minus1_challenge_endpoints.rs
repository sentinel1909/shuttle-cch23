// server/tests/endpoints.rs

// endpoint integration tests for the 2023 Shuttle Christmas Code Hunt Challenge solutions

// day minus 1 challenge
mod minus_1_challenge_endpoint_tests {

    // dependencies
    use cch23_sentinel1909::cch23service::Cch23Service;
    use hyper::body;
    use hyper::body::Body;
    use hyper::Request;
    use tower_test::mock::spawn::Spawn;

    // spawn a router for testing
    fn spawn_service() -> Spawn<Cch23Service> {
        let service = Cch23Service;
        Spawn::new(service)
    }

    // test for the -1 challenge root endpoint
    #[tokio::test]
    async fn test_root() {
        let mut mock = spawn_service();

        let request = Request::builder()
            .uri("/")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 200);
        let body_bytes = body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body_bytes, String::from("").as_bytes());
    }

    // test for the -1 challenge fake error endpoint
    #[tokio::test]
    async fn test_fake_error() {
        let mut mock = spawn_service();

        let request = Request::builder()
            .uri("/-1/error")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 500);
        let body_bytes = body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body_bytes, String::from("Internal Server Error").as_bytes());
    }
}
