// common/tests/day_4_challenge_endpointss.rs

// endpoint integration tests for the 2023 Shuttle Christmas Code Hunt Challenge solutions

// tests for the day 4 challenge grinch endpoints
mod day_4_challenge_endpoint_tests {

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
                        "strength": 5
                    },
                    {
                        "name": "Dancer",
                        "strength": 10
                    },
                    {
                        "name": "Prancer",
                        "strength": 15
                    },
                    {
                        "name": "Vixen",
                        "strength": 20
                    },
                    {
                        "name": "Comet",
                        "strength": 25
                    },
                    {
                        "name": "Cupid",
                        "strength": 30
                    },
                    {
                        "name": "Donner",
                        "strength": 35
                    },
                    {
                        "name": "Blitzen",
                        "strength": 40
                    },
                    {
                        "name": "Rudolph",
                        "strength": 45
                    }
                ]
                "#,
            ))
            .unwrap();

        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 200);
        let body_bytes = body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body_bytes, String::from("225").as_bytes());
    }

    // test for the day 4 challenge get_strength_result endpoint
    #[tokio::test]
    async fn test_get_contest_result() {
        let mut mock = spawn_router();

        let request = Request::builder()
            .uri("/4/contest")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"
                [
                    {
                        "name": "Dasher",
                        "strength": 5,
                        "speed": 50.4,
                        "height": 80,
                        "antler_width": 36,
                        "snow_magic_power": 9001,
                        "favorite_food": "hay",
                        "cAnD13s_3ATeN-yesT3rdAy": 2
                    },
                    {
                        "name": "Dancer",
                        "strength": 6,
                        "speed": 48.2,
                        "height": 65,
                        "antler_width": 37,
                        "snow_magic_power": 4004,
                        "favorite_food": "grass",
                        "cAnD13s_3ATeN-yesT3rdAy": 5
                    }
                ]
                "#,
            ))
            .unwrap();

        let response = mock.call(request);
        let response = response.await.unwrap();
        assert_eq!(response.status(), 200);
        let body_bytes = body::to_bytes(response.into_body()).await.unwrap();
        let obj = json!({
        "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
        "tallest": "Dasher is standing tall with his 36 cm wide antlers",
        "magician": "Dasher could blast you away with a snow magic power of 9001",
        "consumer": "Dancer ate lots of candies, but also some grass"
    }).to_string();
        assert_eq!(
            body_bytes,
            obj
            .as_bytes()
        );
    }
}
