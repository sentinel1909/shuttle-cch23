// server/tests/day_7_challenge_endpointss.rs

// endpoint integration tests for the 2023 Shuttle Christmas Code Hunt Challenge solutions

// tests for the day 7 challenge decode_the_receipe endpoints

mod day_7_challenge_endpoint_tests {

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

    #[tokio::test]
    async fn test_decode_the_receipe() {
        let mut mock = spawn_router();

        let request = Request::builder()
            .uri("/7/decode")
            .method("GET")
            .header(
                "cookie",
                "recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==",
            )
            .body(Body::empty())
            .unwrap();

        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 200);
        let body_bytes = body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(
            body_bytes,
            String::from("\"{\\\"flour\\\":100,\\\"chocolate chips\\\":20}\"").as_bytes()
        );
    }

    #[tokio::test]
    async fn test_bake() {
        let mut mock = spawn_router();

        let request = Request::builder()
          .uri("/7/bake")
          .method("GET")
          .header(
              "cookie",
              "recipe=eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319",
          )
          .body(Body::empty())
          .unwrap();

        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 200);
        let body_bytes = body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(
          body_bytes,
          String::from(
            "{\"recipe\":{\"flour\":95,\"sugar\":50,\"butter\":30,\"baking powder\":10,\"chocolate chips\":50},\"pantry\":{\"flour\":385,\"sugar\":507,\"butter\":2122,\"baking powder\":865,\"chocolate chips\":457}}"
          ).as_bytes());
    }
}
