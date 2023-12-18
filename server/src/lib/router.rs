// src/lib/router.rs

// dependencies
use common_features::{IntoWebResponse, WebRequest, WebResponse};
use hyper::Method;
use std::collections::HashMap;
use tower::util::BoxCloneService;
use tower::ServiceExt;

// type aliases for our router
type RouterKey = (Method, String);
type RouterService = BoxCloneService<WebRequest, WebResponse, WebResponse>;

// a struct type to represent a router
#[derive(Debug, Default, Clone)]
pub struct Router {
    pub endpoints: HashMap<RouterKey, RouterService>,
    pub parameters: Vec<i32>,
}

// implement the Router type
impl Router {
    pub fn on<R, E>(
        &mut self,
        method: Method,
        endpoint: String,
        svc: BoxCloneService<WebRequest, R, E>,
    ) where
        R: IntoWebResponse + 'static,
        E: IntoWebResponse + 'static,
    {
        let svc = BoxCloneService::new(
            svc.map_response(IntoWebResponse::into_web_response)
                .map_err(IntoWebResponse::into_web_response),
        );
        self.endpoints.insert((method, endpoint), svc);
        self.parameters.push(0);
    }
}
