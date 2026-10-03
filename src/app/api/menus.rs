use crate::api::schema::ResponseResult;
use crate::app::App;

use super::responses::encode_success;

impl App {
    pub(super) fn handle_menu_list(&self, id: String) -> String {
        encode_success(
            id,
            ResponseResult::MenuList {
                menus: self.menu_list(),
            },
        )
    }
}
