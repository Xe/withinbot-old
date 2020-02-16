use chrono::prelude::*;
use serde::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Switch {
    pub id: String,
    pub who: String,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub duration: i64,
}
