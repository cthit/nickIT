use rocket_contrib::Json;
use rocket::request::FromParam;
use rocket::http::{RawStr, Status};
use rocket::response::status;
use ldap_hook::user::LdapUser;
use ldap_hook::config::LdapConfig;
use ldap_hook::ldap_search;
use serde_json;

#[derive(PartialEq, Eq)]
pub enum LdapQueryType {
	Nick,
	UID,
}

impl<'a> FromParam<'a> for LdapQueryType {
	type Error = &'a RawStr;
	fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
		if param == "nick" {
			Ok(LdapQueryType::Nick)
		} else if param == "uid" {
			Ok(LdapQueryType::UID)
		} else {
			Err(param)
		}
	}
}

// Dummy endpoints so that the front-end won't complain
#[get("/search/<_query_type>")]
pub fn empty_search(_query_type: LdapQueryType) -> Json<Vec<String>> { Json(vec![]) }

#[get("/search/<query_type>/<query>")]
pub fn search(query_type: LdapQueryType, query: String) -> Result<Json<Vec<LdapUser>>, status::Custom<Json>> {
	let id_list: Vec<String> = match serde_json::from_str(query.as_str()) {
		Ok(l)  => l,
		Err(_) => {
			return Err(status::Custom(
				Status::BadRequest,
				Json(json!({
					"success":false,
					"code":400,
					"reason":"Invalid JSON",
				}))
			)); 
		},
	};

	let mut result_list: Vec<LdapUser> = vec![];

	// TODO: Load the config somewhere more sensible
	let ldap_config = match LdapConfig::load("config.toml") {
		Ok(ldap_config) => ldap_config,
		Err(msg) => {
			println!("Could not load config.toml: {}", msg);
			return Err(status::Custom(
				Status::InternalServerError,
				Json(json!({
					"success":false,
					"code":500,
					"reason":"Internal Server Error",
				}))
			));
		}
	};

	for id in &id_list{
		let filter = match query_type {
			LdapQueryType::UID  => format!("(uid={})", id),
			LdapQueryType::Nick => format!("(nickname=*{}*)", id), // Basic fuzzy search when querying by nicknames
		};
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

	result_list.sort_by(|a, b| match query_type {
		LdapQueryType::UID  => a.uid .cmp(&b.uid ),
		LdapQueryType::Nick => a.nick.cmp(&b.nick),
	});

	Ok(Json(result_list))
}
