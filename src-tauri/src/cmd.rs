
use strum_macros::{EnumString, EnumVariantNames};

#[derive(Deserialize, EnumString, EnumVariantNames, Debug)]
#[serde(tag = "cmd", rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum Cmd {
  MyCustomCommand { msg: String },
}
