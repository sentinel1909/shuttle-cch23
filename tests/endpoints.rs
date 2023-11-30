// tests/endpoints.rs

// tests for Challenge -1
mod endpoint_tests {

    use cch23_sentinel1909::router::Router;
    use hyper::body::Body;
    use hyper::Request;
    use tower_test::mock::spawn::Spawn;   

    #[tokio::test]
    async fn test_root() {
        let router = Router::create();
        let mut mock = Spawn::new(router.clone());

        let request = Request::builder()
            .uri("/")
            .body(Body::empty())
            .unwrap();
        
        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_fake_error() {
        let router = Router::create();
        let mut mock = Spawn::new(router.clone());

        let request = Request::builder()
            .uri("/-1/error")
            .body(Body::empty())
            .unwrap();
        
        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 500);
    }
}
