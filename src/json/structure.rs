use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ResponseHandler {
    pub id: u32,
    pub owner: String,
    pub tags: String, //Just make it easier with array please.
    #[serde(rename = "rating")]
    pub rate: String,
    #[serde(rename = "source")]
    pub original: String,
}