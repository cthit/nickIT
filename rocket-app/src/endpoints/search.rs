use rocket_contrib::Json;
use rocket::request::FromParam;
use rocket::http::RawStr;
use ldap_hook::user::LdapUser;
use ldap_hook::config::LdapConfig;
use ldap_hook::ldap_search;

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
#[get("/search/<query_type>")]
pub fn empty_search(query_type: LdapQueryType) -> Json<Vec<String>> { Json(vec![]) }

#[get("/search/<query_type>/<query>")]
pub fn search(query_type: LdapQueryType, query: String) -> Json<Vec<LdapUser>> {
	let id_list: Vec<String> = query.split(",").map(|s| s.to_string()).collect();

	let mut result_list: Vec<LdapUser> = vec![];

	// TODO: Load the config somewhere more sensible
	let ldap_config = match LdapConfig::load("config.toml") {
		Ok(ldap_config) => ldap_config,
		Err(msg) => {
			panic!("Could not load config.toml: {}", msg);
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

	Json(result_list)
}
