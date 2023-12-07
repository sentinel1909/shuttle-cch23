// common/tests/endpoints.rs

// endpoint integration tests for the 2023 Shuttle Christmas Code Hunt Challenge solutions
mod endpoint_tests {

    // dependencies
    use cch23_sentinel1909::router::Router;
    use hyper::body;
    use hyper::body::Body;
    use hyper::Request;
    use tower_test::mock::spawn::Spawn;

    // spawn a router for testing
    fn spawn_router() -> Spawn<Router> {
        let router = Router::create();
        Spawn::new(router)
    }

    // test for the -1 challenge root endpoint
    #[tokio::test]
    async fn test_root() {
        let mut mock = spawn_router();

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
        let mut mock = spawn_router();

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

    // tests for the day 1 challenge calibrate_sled_ids endpoint
    #[tokio::test]
    async fn test_calibrate_sled_ids() {
        let mut mock = spawn_router();

        let request = Request::builder()
            .uri("/1/4/5/8/10")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 200);
        let body_bytes = body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body_bytes, String::from("27").as_bytes());
    }

    // test for the day 1 challenge calibrate_packet_ids endpoint
    #[tokio::test]
    async fn test_calibrate_packet_ids() {
        let mut mock = spawn_router();

        let request = Request::builder()
            .uri("/1/4/8")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 200);
        let body_bytes = body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body_bytes, String::from("1728").as_bytes());
    }

    // test for the day 4 challenge get_strength_result endpoint
    #[tokio::test]
    async fn test_get_strength() {
        let mut mock = spawn_router();

        let request = Request::builder()
            .uri("/4/strength")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"
                [
                    {
                        "name": "Dasher",
                        "strength": 6
                    },
                    {
                        "name": "Dancer",
                        "strength": 8
                    },
                    {
                        "name": "Prancer",
                        "strength": 14
                    }
                ]
                "#,
            ))
            .unwrap();

        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 200);
        let body_bytes = body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(
            body_bytes,
            String::from("Combined Reindeer Strength: 28").as_bytes()
        );
    }

    // test for the day 4 challenge get_strength_result endpoint
    #[tokio::test]
    async fn test_get_contest_result() {
        let mut mock = spawn_router();

        let request = Request::builder()
            .uri("/4/contest")
            .method("POST")
            .body(Body::empty())
            .unwrap();

        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 200);
        let body_bytes = body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(
            body_bytes,
            String::from("Endpoint not implemented yet!").as_bytes()
        );
    }
}
