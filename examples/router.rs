extern crate iron;
extern crate iron_mountrouter;

// To run, $ cargo run --example router
// To use, go to http://127.0.0.1:3000/
use std::fs::File;
use std::io::Read;

use iron::{Iron, Request, Response, IronResult};
use iron::headers::ContentType;
use iron::status;
use iron_mountrouter::{Router, StrippedUrl};


fn get_output(content: &str) -> String {
	let mut res = String::new();
	File::open("examples/router.html").unwrap().read_to_string(&mut res).unwrap();
	res.replace("<!-- content -->", content)
}

fn main() {
    let mut router = Router::new();
    router.add_route("/", handler, false);
    router.add_route("/:query/:sub-query/", handler, false);
    
    let mut book_router = Router::new();
    book_router.add_route("/page/:key/", handler, false);
    book_router.add_route("/contents/", handler, false);
    router.add_route("/book/:book-name/", book_router, true);

    Iron::new(router).http("localhost:3000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap();
        let mut res = Response::with((
			status::Ok,
			get_output(
				&format!(
					"<p>Url: {:?}<p>Query parts: {:?}<p>Stripped url: {:?}",
					req.url.path,
					*query,
					req.extensions.get::<StrippedUrl>()
				)
			)
		));
    	res.headers.set(ContentType::html());
    	Ok(res)
    }
}