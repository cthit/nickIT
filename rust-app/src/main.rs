#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate ldap3;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs::File;
use std::io::Read;
use rocket_contrib::Json;
use ldap3::{LdapConn, Scope, SearchEntry};
use std::error::Error;

#[derive(Deserialize)]
struct LdapConfig {
	url: String,
	base_dn: String,
//	filter: String,
	attributes: Vec<String>,
	user_dn: String,
	password: String,
}

fn ldap_search(config: &LdapConfig, filter: &str) -> Result<Vec<String>, Box<Error>> {
	let ldap = LdapConn::new(config.url.as_str())?;

	ldap.simple_bind(config.user_dn.as_str(), config.password.as_str())?;

	let (rs, _, _) = ldap.search(
		config.base_dn.as_str(),
		Scope::Subtree,
		filter,
		config.attributes.clone(),
	)?;

	let mut list: Vec<String> = vec![];

	for entry in rs {
		list.push(SearchEntry::construct(entry).dn);
	}

	Ok(list)
}

fn get_ldap_config() -> Result<LdapConfig, Box<Error>> {
	let mut config_file = File::open("config.toml")?;
	let mut config_content = String::new();
	config_file.read_to_string(&mut config_content)?;
	let ldap_config: LdapConfig = toml::from_str(config_content.as_str())?;
	Ok(ldap_config)
}

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
	let nick_list: Vec<String> = query.split(",").map(|s| s.to_string()).collect();

	let mut result_list: Vec<String> = vec![];

	// TODO: Load the config somewhere more sensible
	let ldap_config = match get_ldap_config() {
		Ok(ldap_config) => ldap_config,
		Err(msg) => {
			println!("Could not load config.toml: {}", msg);
			panic!();
		}
	};

	for nick in &nick_list{
		let filter = format!("(nickname={})", nick); // TODO: Use ldap_config.filter
		let res = ldap_search(&ldap_config, filter.as_str());
	
		match res.ok() {
			None => { break; }
			Some(list) => {
				for entry in list {
					if !result_list.contains(&entry) {
						result_list.push(entry)
					}
				}
			}
		}
	}
	
	Json(result_list)
}

fn main() {
	rocket::ignite().mount("/", routes![
		root,
		empty_search,
		search,
	]).launch();
}
