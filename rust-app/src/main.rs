#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::Json;

#[get("/")]
fn root() -> String {
	String::from("Well hi there!")
}

// Dummy endpoint so that the front-end won't complain
#[get("/search")]
fn empty_search() -> Json<Vec<String>> {
	Json(vec![])
}

#[get("/search/<query>")]
fn search(query: String) -> Json<Vec<String>> {
	let nick_list = query.split(",").map(|s| s.to_string()).collect();
	return Json(nick_list);
}

fn main() {
	rocket::ignite().mount("/", routes![
		root,
		search,
		empty_search,
	]).launch();
}
