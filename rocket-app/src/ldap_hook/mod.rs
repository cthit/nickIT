pub mod config;

use ldap_hook::config::LdapConfig;
use ldap3::{LdapConn, Scope, SearchEntry};
use std::error::Error;

pub fn ldap_search(config: &LdapConfig, filter: &str) -> Result<Vec<String>, Box<Error>> {
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
