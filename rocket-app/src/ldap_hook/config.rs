extern crate toml;

use std::fs::File;
use std::io::Read;
use std::error::Error;

#[derive(Deserialize)]
pub struct LdapConfig {
	pub url:        String,
	pub base_dn:    String,
	pub user_dn:    String,
	pub password:   String,
}

impl LdapConfig {
	pub fn load(file_name: &str) -> Result<LdapConfig, Box<Error>> {
		let mut config_file = File::open(file_name)?;
		let mut config_content = String::new();
		config_file.read_to_string(&mut config_content)?;
		let ldap_config: LdapConfig = toml::from_str(config_content.as_str())?;
		Ok(ldap_config)
	}
}
