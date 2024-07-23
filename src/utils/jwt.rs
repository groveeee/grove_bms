use serde::{Deserialize, Serialize};

/// jwt工具类

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) aud: String,
    pub(crate) sub: String,
    pub(crate) company: String,
    pub(crate) exp: u64,
}
impl Claims {
    pub fn new(aud: String, sub: String, company: String, exp: u64) -> Self {
        Self { aud, sub, company, exp }
    }
}