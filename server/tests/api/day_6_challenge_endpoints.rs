// common/tests/day_6_challenge_endpointss.rs

// endpoint integration tests for the 2023 Shuttle Christmas Code Hunt Challenge solutions

// tests for the day 6 challenge decode_the_receipe endpoints

mod day_6_challenge_endpoint_tests {
    // dependencies
    use cch23_sentinel1909::router::Router;
    use hyper::body;
    use hyper::body::Body;
    use hyper::Request;
    use serde_json::json;
    use tower_test::mock::spawn::Spawn;

    // spawn a router for testing
    fn spawn_router() -> Spawn<Router> {
        let router = Router::create();
        Spawn::new(router)
    }

    // test for the day 6, part 1 challenge
    #[tokio::test]
    async fn test_count_elf() {
        let mut mock = spawn_router();

        let request = Request::builder()
            .uri("/6")
            .method("POST")
            .header("content-type", "text/plain")
            .body(Body::from(
                "The mischievous elf peeked out from behind the toy workshop, and another elf joined in the festive dance. Look, there is also an elf on that shelf!",
            ))
            .unwrap();

        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 200);
        let body_bytes = body::to_bytes(response.into_body()).await.unwrap();
        let obj = json!({
            "elf": 4,
            "elf on a shelf": 0,
            "shelf with no elf on it": 0,
        })
        .to_string();
        assert_eq!(body_bytes, obj.as_bytes());
    }

    // test for the day 6, part 2 challenge (elf on a shelf)
    #[tokio::test]
    async fn test_count_elf_on_a_shelf() {
        let mut mock = spawn_router();

        let request = Request::builder()
            .uri("/6")
            .method("POST")
            .header("content-type", "text/plain")
            .body(Body::from(
                "there is an elf on a shelf on an elf. there is also another shelf in Belfast.",
            ))
            .unwrap();

        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 200);
        let body_bytes = body::to_bytes(response.into_body()).await.unwrap();
        let obj = json!({
            "elf":5,
            "elf on a shelf": 1,
            "shelf with no elf on it": 1
        })
        .to_string();
        assert_eq!(body_bytes, obj.as_bytes());
    }

    // test for the day 6, part 2 challenge (shelves with no elves)
    #[tokio::test]
    async fn test_count_shelves_with_no_elves() {
        let mut mock = spawn_router();

        let request = Request::builder()
            .uri("/6")
            .method("POST")
            .header("content-type", "text/plain")
            .body(Body::from(
                "there is an elf on a shelf on an elf. there is also another shelf in Belfast.",
            ))
            .unwrap();

        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 200);
        let body_bytes = body::to_bytes(response.into_body()).await.unwrap();
        let obj = json!({
            "elf":5,
            "elf on a shelf": 1,
            "shelf with no elf on it": 1
        })
        .to_string();
        assert_eq!(body_bytes, obj.as_bytes());
    }
}
