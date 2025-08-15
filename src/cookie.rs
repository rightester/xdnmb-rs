use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct UserCookie {
    pub name: String,

    #[serde(rename="cookie")]
    pub value: String,
}

impl UserCookie {
    pub fn new(name: &str, value: &str) -> Self {
        Self { name: name.to_string(), value: value.to_string() }
    }
}
