use rocket_contrib::Json;
use ldap_hook::user::LdapUser;
use ldap_hook::config::LdapConfig;
use ldap_hook::ldap_search;

// Dummy endpoints so that the front-end won't complain
#[get("/search/nick")]
pub fn empty_search_nick() -> Json<Vec<String>> { Json(vec![]) }

#[get("/search/uid")]
pub fn empty_search_uid()  -> Json<Vec<String>> { Json(vec![]) }

fn search(query: String, by_uid: bool) -> Json<Vec<LdapUser>> {
	let id_list: Vec<String> = query.split(",").map(|s| s.to_string()).collect();

	let mut result_list: Vec<LdapUser> = vec![];

	// TODO: Load the config somewhere more sensible
	let ldap_config = match LdapConfig::load("config.toml") {
		Ok(ldap_config) => ldap_config,
		Err(msg) => {
			println!("Could not load config.toml: {}", msg);
			panic!();
		}
	};

	for id in &id_list{
		let filter = if by_uid {
			format!("(uid={})", id)
		} else {
			format!("(nickname=*{}*)", id)
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

	Json(result_list)
}

#[get("/search/nick/<query>")]
pub fn search_nick(query: String) -> Json<Vec<LdapUser>> {
	search(query, false)
}

#[get("/search/uid/<query>")]
pub fn search_uid(query: String) -> Json<Vec<LdapUser>> {
	search(query, true)
}
