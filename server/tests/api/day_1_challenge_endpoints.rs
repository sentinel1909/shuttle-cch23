// server/tests/day_4_challenge_endpointss.rs

// endpoint integration tests for the 2023 Shuttle Christmas Code Hunt Challenge solutions

// tests for the day 4 challenge grinch endpoints
mod day_1_challenge_endpoint_tests {

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
}
