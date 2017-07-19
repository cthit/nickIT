use rocket_contrib::Json;
use ldap_hook::config::LdapConfig;
use ldap_hook::ldap_search;

// Dummy endpoint so that the front-end won't complain
#[get("/search")]
pub fn empty_search() -> Json<Vec<String>> {
	Json(vec![])
}

#[get("/search/<query>")]
pub fn search(query: String) -> Json<Vec<String>> {
	let nick_list: Vec<String> = query.split(",").map(|s| s.to_string()).collect();

	let mut result_list: Vec<String> = vec![];

	// TODO: Load the config somewhere more sensible
	let ldap_config = match LdapConfig::load("config.toml") {
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
