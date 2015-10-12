use std::error::Error;
use std::fmt;

use iron::{Request, Response, Handler, IronResult, IronError};
use iron::status;
use iron::method::Method;

type PickerHandlers = Vec<(Option<Method>, Box<Handler>)>;


/// `MethodPicker` provides an interface to apply different handlers for different http methods as
/// middleware for the Iron framework.
pub struct MethodPicker {
	handlers: PickerHandlers
}

impl MethodPicker {
    /// Construct new `MethodPicker`.
    ///
    /// ```
    /// # use iron_mountrouter::MethodPicker;
    /// let picker = MethodPicker::new();
    /// ```
	pub fn new() -> MethodPicker {
		MethodPicker {
			handlers: PickerHandlers::new()
		}
	}
	
	/// Adds new handler to picker
	///
	/// `method` is any Iron's http method.
	///
	/// `handler` will be used for specified method
	pub fn add<H: Handler>(&mut self, method: Method, handler: H) -> &mut MethodPicker {
		self.handlers.push((Some(method), Box::new(handler)));
		self
	}
	
	/// Adds default handler to picker which will be used if no method matched
	pub fn default<H: Handler>(&mut self, handler: H) -> &mut MethodPicker {
		self.handlers.push((None, Box::new(handler)));
		self
	}
}

impl Handler for MethodPicker {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
    	for &(ref method, ref handler) in self.handlers.iter() {
    		match method {
    			&Some(ref m) => if *m == req.method {
    				return handler.handle(req);
    			},
    			&None => return handler.handle(req)
    		}
    	}
    	Err(IronError::new(NoMethod, status::NotFound))
    }
}


/// `NoMethod` is error type returned to Iron if no handler was matched in MethodPicker
#[derive(Debug)]
pub struct NoMethod;

impl fmt::Display for NoMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("No matching method found.")
    }
}

impl Error for NoMethod {
    fn description(&self) -> &str { "No method" }
}