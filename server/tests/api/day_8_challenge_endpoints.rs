// server/tests/day_8_challenge_endpointss.rs

// endpoint integration tests for the 2023 Shuttle Christmas Code Hunt Challenge solutions

// tests for the day 8 challenge decode_the_receipe endpoints

mod day_8_challenge_endpoint_tests {

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

    // test the get_weight endpoint
    #[tokio::test]
    async fn test_get_weight() {
        // spawn a router for testing
        let mut mock = spawn_router();

        // create a request
        let request = Request::builder()
            .uri("/8/weight/25")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 200);
        let body_bytes = body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body_bytes, "6".as_bytes())
    }
}
