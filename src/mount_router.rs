use std::error::Error;
use std::fmt;
use std::collections::BTreeMap;

use iron::{Request, Response, Handler, IronResult, IronError, Url};
use iron::status;
use iron::typemap::Key;

use recognizer::Router as Recognizer;
use recognizer::Params as RecognizerParams;
use recognizer::Match;

use url::parse_path;

const LEFTOVER_PARAM_NAME: &'static str = "mountrouter_leftover";
pub type Params = BTreeMap<String, String>;

struct RouteParams {
	handler: Box<Handler>,
	is_mounted: bool,
}

pub struct Router {
    recognizer: Recognizer<RouteParams>
}

impl Router {
    pub fn new() -> Router {
        Router {
            recognizer: Recognizer::new()
        }
    }

    pub fn add_route<H, S>(&mut self, route: S, handler: H, is_mounted: bool) -> &mut Router
    where H: Handler, S: AsRef<str> {
    	let params = RouteParams {
    		handler: Box::new(handler),
    		is_mounted: is_mounted
    	};
    	if is_mounted {
    		let r = route.as_ref();
    		let mut modified_route = String::with_capacity(r.len() + 2 + LEFTOVER_PARAM_NAME.len());
    		modified_route.push_str(r);
    		if !r.ends_with('/') {
				modified_route.push('/');
			}
    		modified_route.push('*');
    		modified_route.push_str(LEFTOVER_PARAM_NAME);
	        self.recognizer.add(&modified_route, params);
    	} else {
	        self.recognizer.add(route.as_ref(), params);
    	}
        self
    }

    fn recognize(&self, path: &str) -> Option<Match<&RouteParams>> {
        self.recognizer.recognize(path).ok()
    }
    
    fn append_params(req: &mut Request, params: &RecognizerParams) {
    	if !req.extensions.contains::<Router>() {
			req.extensions.insert::<Router>(Params::new());
    	}
    	let p = req.extensions.get_mut::<Router>().unwrap();
    	for name in params.keys() {
    		if name != LEFTOVER_PARAM_NAME {
    			p.insert(name.to_string(), params[name.as_ref()].clone());
			}
    	}
    }
}

impl Key for Router { type Value = Params; }

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let path = req.url.path.join("/");
        match self.recognize(&path) {
        	Some(matched) => {
	            if matched.handler.is_mounted {
	            	let leftover = "/".to_string() + &matched.params[LEFTOVER_PARAM_NAME];
	            	let (new_path, _, _) = parse_path(&leftover).unwrap();
			        if !req.extensions.contains::<OriginalUrl>() {
			            req.extensions.insert::<OriginalUrl>(req.url.clone());
			        }
	            	req.url.path = new_path;
	            }
            	Router::append_params(req, &matched.params);
	            matched.handler.handler.handle(req)
        	},
        	None => Err(IronError::new(NoRoute, status::NotFound))
        }
    }
}

#[derive(Copy, Clone)]
pub struct OriginalUrl;
impl Key for OriginalUrl { type Value = Url; }

#[derive(Debug)]
pub struct NoRoute;

impl fmt::Display for NoRoute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("No matching route found.")
    }
}

impl Error for NoRoute {
    fn description(&self) -> &str { "No Route" }
}