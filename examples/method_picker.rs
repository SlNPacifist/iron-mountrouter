extern crate iron;
extern crate iron_mountrouter;

// To run, $ cargo run --example method_picker
// To use, go to http://127.0.0.1:3000/

use std::fs::File;
use std::io::Read;

use iron::{Iron, Request, Response, IronResult};
use iron::headers::ContentType;
use iron::status;
use iron::method;

use iron_mountrouter::MethodPicker;


fn get_output(content: &str) -> String {
	let mut res = String::new();
	File::open("examples/method_picker.html").unwrap().read_to_string(&mut res).unwrap();
	res.replace("<!-- content -->", content)
}

fn main() {
    let mut picker = MethodPicker::new();
    picker.add(method::Get, handler);
    picker.default(default_handler);

    Iron::new(picker).http("localhost:3000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
    	let mut res = Response::with((
			status::Ok,
			get_output(&format!("method used: {:?}", req.method))
		));
	    res.headers.set(ContentType::html());
        Ok(res)
    }
    
    fn default_handler(req: &mut Request) -> IronResult<Response> {
    	let mut res = Response::with((
			status::NotFound,
			get_output(
				&format!("Default handler is used, method used: {:?}", req.method)
			)
		));
    	res.headers.set(ContentType::html());
    	Ok(res)
    }
}