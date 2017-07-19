#![feature(plugin)]
#![plugin(rocket_codegen)]

mod ldap_hook;
mod endpoints;

extern crate rocket;
extern crate rocket_contrib;
extern crate ldap3;
#[macro_use]
extern crate serde_derive;

fn main() {
	rocket::ignite().mount("/", routes![
		endpoints::search::empty_search_nick,
		endpoints::search::search_nick,
		endpoints::search::empty_search_uid,
		endpoints::search::search_uid,
	]).launch();
}
