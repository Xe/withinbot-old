use serde::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Switch {
  pub id: String,
  pub who: String,
  pub started_at: String,
  pub duration: i64,
}
