#![feature(plugin)]
#![plugin(rocket_codegen)]

mod ldap_hook;
mod endpoints;
mod util;

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate ldap3;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use endpoints::search::*;
use util::cors::CORS;

fn main() {
	rocket::ignite().mount("/", routes![
		empty_search,
		search,
	]).attach(CORS())
	  .launch();
}
