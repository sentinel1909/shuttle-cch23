// common/tests/day_6_challenge_endpointss.rs

// endpoint integration tests for the 2023 Shuttle Christmas Code Hunt Challenge solutions

// tests for the day 6 challenge decode_the_receipe endpoints

mod day_6_challenge_endpoint_tests {
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

    // test for the day 6 challenge count_elf endpoint
    #[tokio::test]
    async fn test_count_elf() {
        let mut mock = spawn_router();

        let request = Request::builder()
            .uri("/6")
            .method("POST")
            .header("content-type", "text/plain")
            .body(Body::from(
                "The mischievous elf peeked out from behind the toy workshop, and another elf joined in the festive dance. Look, there is also an elf on that shelf! There is another elf hiding in the shadows nearby. The elves are everywhere!",
            ))
            .unwrap();

        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 200);
        let body_bytes = body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body_bytes, String::from("\"elf:5\"").as_bytes());
    }
}