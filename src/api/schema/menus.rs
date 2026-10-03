use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct MenuItemInfo {
    pub command_id: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hotkey: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub divider: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct MenuInfo {
    pub menu_id: String,
    pub binding_label: String,
    pub binding_labels: Vec<String>,
    pub title: String,
    pub items: Vec<MenuItemInfo>,
}
