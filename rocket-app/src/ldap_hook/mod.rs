pub mod config;
pub mod user;

use ldap_hook::config::LdapConfig;
use ldap_hook::user::LdapUser;
use ldap3::{LdapConn, Scope, SearchEntry};
use std::error::Error;

pub fn ldap_search(config: &LdapConfig, filter: &str) -> Result<Vec<LdapUser>, Box<Error>> {
	let ldap = LdapConn::new(config.url.as_str())?;

	ldap.simple_bind(config.user_dn.as_str(), config.password.as_str())?;

	let (rs, _, _) = ldap.search(
		config.base_dn.as_str(),
		Scope::Subtree,
		filter,
		config.attributes.clone(),
	)?;

	let mut list: Vec<LdapUser> = vec![];

	for entry in rs {
		list.push(LdapUser::from(&SearchEntry::construct(entry)));
	}

	Ok(list)
}
