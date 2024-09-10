use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BusinessSearch {
    pub q: String,
}
