use ldap3::SearchEntry;

#[derive(Serialize)]
pub struct LdapUser {
	pub uid: String,
	pub nick: String,
}

impl LdapUser {
	pub fn from(sr: &SearchEntry) -> LdapUser {
		LdapUser {
			uid: sr.attrs["uid"][0].clone(),
			nick: sr.attrs["nickname"][0].clone(),
		}
	}
}

impl PartialEq for LdapUser {
	fn eq(&self, other: &LdapUser) -> bool {
		self.uid == other.uid
	}
}
