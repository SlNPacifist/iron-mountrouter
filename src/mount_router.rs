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

/// `Params` is stored in request extensions and gives access to captured dynamic parameters
///
/// ```ignore
/// let ref params = req.extensions.get::<Router>().unwrap()
/// ```
pub type Params = BTreeMap<String, String>;

struct RouteParams {
	handler: Box<Handler>,
	is_mounted: bool,
}

/// `Router` provides an interface to apply different handlers to different routes as middleware for
/// the Iron framework.
pub struct Router {
    recognizer: Recognizer<RouteParams>
}

impl Router {
    /// Construct new `Router`.
    ///
    /// ```
    /// # use iron_mountrouter::Router;
    /// let router = Router::new();
    /// ```
    pub fn new() -> Router {
        Router {
            recognizer: Recognizer::new()
        }
    }

    /// Add a new route to a `Router`, matching given pattern.
    ///
    /// `route` is a pattern supported by [route recognizer]
    /// (https://crates.io/crates/route-recognizer):
    ///
    /// string like `/static/page/here` is matched entirely;
    ///
    /// `:name` is a dynamic param, matching any string that does not contain slash `/`, i.e.
    ///
    /// `/page/:page-num/content` will match `/page/17/content` or `/page/abbra-t/content`
    /// but not `/page/17/5/content`.
    ///
    /// Colon only works at the start of the segment so `/page-:num` works like usual string match
    /// and will not match `/page-5`.
    ///
    /// Currently there is no way to set param type so it catches only strings and does not perform
    /// any additional checks.
    ///
    /// `*name` is a dynamic wildcard param, matching any string (even those containing slashes`/`).
    /// `/redirect/to/*path` will match `/redirect/to/example.com/example`.
    ///
    /// Captured parameters are stored in [Params](type.Params.html) type and can be accessed
    /// in request extensions:
    ///
    /// ```ignore
    /// let ref params = req.extensions.get::<Router>().unwrap()
    /// ```
    ///
    /// Unwrap is safe here because router always adds extension if route is matched, even if no
    /// params were captured.
    ///
    /// `handler` is any [Iron handler]
    /// (http://ironframework.io/doc/iron/middleware/trait.Handler.html). Working with mountrouter
    /// you will most often use [MethodPicker](struct.MethodPicker.html) to pick appropriate
    /// handler for given http method.
    ///
    /// If `is_mounted` is true, router will strip matched part of the request url. Unlike [Mount]
    /// (http://ironframework.io/doc/mount/index.html) middleware from the Iron's core bundle, this
    /// parameter allows you to strip dynamical (e.g. with dynamic params) strings from request
    /// path. Original url is stored in request extensions using `OriginalUrl`. For example,
    ///
    /// ```ignore
    /// let mut router = Router::new();
    /// router.add_route("/book/:page/", handler, true);
    /// fn handler(req: &mut Request) -> IronResult<Response> {
    ///     // for path "/book/17/page/3"
    ///
    ///     // ["page", "3"]
    ///     println!("{:?}", req.url.path);
    ///     // ["book", "17", "page", "3"]
    ///     println!("{:?}", req.extensions.get::<OriginalUrl>().unwrap().path);
    /// }
    /// ```
    /// If `is_mounted` is set, route is forced to end with slash: slash is appended to the end of
    /// route if it is not there.
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


/// `OriginalUrl` serves as a key in request extensions to original url if router strips url for
/// mounted handlers.
///
/// ```ignore
/// let ref original_url = req.extensions.get::<OriginalUrl>()
/// ```
#[derive(Copy, Clone)]
pub struct OriginalUrl;
impl Key for OriginalUrl { type Value = Url; }


/// `NoRoute` is error type returned to Iron if no route was matched in Router
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