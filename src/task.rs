use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Task {
    pub command: String,

    #[serde(default)]
    pub depends: Vec<String>,
}
