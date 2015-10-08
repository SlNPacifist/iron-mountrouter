extern crate iron;
extern crate route_recognizer as recognizer;

use std::error::Error;
use std::fmt;

use iron::{Request, Response, Handler, IronResult, IronError};
use iron::status;
use iron::typemap::Key;

use recognizer::Router as Recognizer;
use recognizer::{Match, Params};


pub struct Router {
    recognizer: Recognizer<Box<Handler>>
}

impl Router {
    pub fn new() -> Router {
        Router {
            recognizer: Recognizer::new()
        }
    }

    pub fn add_route<H, S>(&mut self, route: S, handler: H) -> &mut Router
    where H: Handler, S: AsRef<str> {
        self.recognizer.add(route.as_ref(), Box::new(handler));
        self
    }

    fn recognize(&self, path: &str) -> Option<Match<&Box<Handler>>> {
        self.recognizer.recognize(path).ok()
    }
}

impl Key for Router { type Value = Params; }

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let path = req.url.path.join("/");
        match self.recognize(&path) {
        	Some(matched) => {
	            req.extensions.insert::<Router>(matched.params);
	            matched.handler.handle(req)
        	},
        	None => Err(IronError::new(NoRoute, status::NotFound))
        }
    }
}

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