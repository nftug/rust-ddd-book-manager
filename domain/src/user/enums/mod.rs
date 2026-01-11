use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, EnumString};

#[derive(
    Debug, EnumString, AsRefStr, EnumIter, Default, PartialEq, Eq, Clone, Deserialize, Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Admin,
    #[default]
    Regular,
    System,
}
