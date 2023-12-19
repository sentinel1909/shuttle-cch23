// src/lib/router.rs

// dependencies
use common_features::{IntoWebResponse, WebRequest, WebResponse};
use hyper::Method;
use std::collections::HashMap;
use tower::util::BoxCloneService;
use tower::ServiceExt;

// type aliases for our router
type RouterService = BoxCloneService<WebRequest, WebResponse, WebResponse>;

// a struct type to represent a router
#[derive(Debug, Default, Clone)]
pub struct Router {
    pub endpoints: Vec<RouterEndpoint>,
}

// struct to represent the incoming URI parameters
#[derive(Debug, Clone, Default)]
pub struct UriParams {
    pub params: Option<HashMap<String, String>>,
}

impl UriParams {
    pub fn insert(&mut self, name: String, value: String) {
        self.params
            .get_or_insert_with(HashMap::new)
            .insert(name, value);
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<&str> {
        self.params
            .as_ref()
            .and_then(|params| params.get(name.as_ref()))
            .map(String::as_str)
    }
}

// struct to represent a router endpoint, contains a matcher and a service
#[derive(Debug, Clone)]
pub struct RouterEndpoint {
    pub matcher: EndpointMatcher,
    pub service: RouterService,
}

impl RouterEndpoint {
    pub(crate) fn new(method: Method, path: &'static str, service: RouterService) -> Self {
        Self {
            matcher: EndpointMatcher::new(method, path),
            service,
        }
    }
}

// an enum to represent a path fragment
#[derive(Debug, Clone)]
enum PathFragment {
    Literal(&'static str),
    Param(&'static str),
}

// a struct to represent the endpoint matcher
#[derive(Debug, Clone)]
pub struct EndpointMatcher {
    pub fragments: Vec<PathFragment>,
    pub method: Method,
}

impl EndpointMatcher {
    pub fn new(method: Method, path: &'static str) -> Self {
        let fragments: Vec<PathFragment> = path
            .split('/')
            .filter_map(|s| {
                if s.is_empty() {
                    return None;
                }
                if s.starts_with(':') {
                    Some(PathFragment::Param(s.trim_start_matches(':')))
                } else {
                    Some(PathFragment::Literal(s))
                }
            })
            .collect();
        Self { fragments, method }
    }

    pub fn match_request(&self, method: &Method, path: &str) -> Option<UriParams> {
        if method != self.method {
            return None;
        }

        let fragments_iter = self
            .fragments
            .iter()
            .map(Some)
            .chain(std::iter::repeat(None));

        let mut params = UriParams::default();

        for (segment, fragment) in path.split('/').map(Some).zip(fragments_iter) {
            match (segment, fragment) {
                (Some(segment), Some(fragment)) => match fragment {
                    PathFragment::Literal(literal) => {
                        if !literal.eq_ignore_ascii_case(segment) {
                            return None;
                        }
                    }
                    PathFragment::Param(name) => {
                        params.insert(name.to_string(), segment.to_string());
                    }
                },
                (None, None) => {
                    break;
                }
                _ => {
                    return None;
                }
            }
        }

        Some(params)
    }
}

// implement the Router type
impl Router {
    pub fn on<R, E>(
        &mut self,
        method: Method,
        endpoint: &'static str,
        svc: BoxCloneService<WebRequest, R, E>,
    ) where
        R: IntoWebResponse + 'static,
        E: IntoWebResponse + 'static,
    {
        let svc = BoxCloneService::new(
            svc.map_response(IntoWebResponse::into_web_response)
                .map_err(IntoWebResponse::into_web_response),
        );
        self.endpoints
            .push(RouterEndpoint::new(method, endpoint, svc));
    }
}
