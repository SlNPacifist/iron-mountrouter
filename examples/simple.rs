extern crate iron;
extern crate iron_mountrouter;

// To run, $ cargo run --example simple
// To use, go to http://127.0.0.1:3000/test

use iron::{Iron, Request, Response, IronResult};
use iron::status;
use iron_mountrouter::{Router, OriginalUrl};

fn main() {
    let mut router = Router::new();
    router.add_route("/", handler, false);
    router.add_route("/:query/:sub-query", handler, false);
    
    let mut internal_router = Router::new();
    internal_router.add_route("/delete/:key/", handler, false);
    internal_router.add_route("/update/:key/", handler, false);
    router.add_route("/page/:num/", internal_router, true);

    Iron::new(router).http("localhost:3000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap();
        Ok(Response::with((
			status::Ok,
			format!(
				"Url: {:?}\nQuery params: {:?}\nOriginal url: {:?}",
				req.url.path,
				*query,
				req.extensions.get::<OriginalUrl>()
			)
		)))
    }
}