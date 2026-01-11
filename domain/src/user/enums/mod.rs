use strum::{AsRefStr, EnumIter, EnumString};

#[derive(Debug, EnumString, AsRefStr, EnumIter, Default, PartialEq, Eq, Clone)]
pub enum UserRole {
    Admin,
    #[default]
    Regular,
    System,
}
