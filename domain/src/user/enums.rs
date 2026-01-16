use strum::{AsRefStr, EnumString};

#[derive(Debug, EnumString, AsRefStr, PartialEq, Eq, Clone, Copy)]
#[strum(ascii_case_insensitive)]
pub enum UserRole {
    Admin,
    Regular,
    System,
}
