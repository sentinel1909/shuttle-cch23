// common/tests/day_5_challenge_endpointss.rs

// endpoint integration tests for the 2023 Shuttle Christmas Code Hunt Challenge solutions

// tests for the day 5 challenge grinch endpoints

mod day_5_challenge_endpoint_tests {
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

    // test for the day 5 challenge count_grinch endpoint
    #[tokio::test]
    async fn test_count_grinch() {
        let mut mock = spawn_router();

        let request = Request::builder()
            .uri("/5/grinch")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 200);
        let body_bytes = body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body_bytes, String::from("You're a mean one, Mr. Grinch!").as_bytes());
    }
}