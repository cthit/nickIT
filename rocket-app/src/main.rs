#![feature(plugin)]
#![plugin(rocket_codegen)]

mod ldap_hook;
mod endpoints;
mod util;

extern crate rocket;
extern crate rocket_contrib;
extern crate ldap3;
#[macro_use]
extern crate serde_derive;

use util::cors::CORS;

fn main() {
	rocket::ignite().mount("/", routes![
		endpoints::search::empty_search_nick,
		endpoints::search::search_nick,
		endpoints::search::empty_search_uid,
		endpoints::search::search_uid,
	]).attach(CORS())
	  .launch();
}
