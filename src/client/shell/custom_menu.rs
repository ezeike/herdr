use super::*;

impl ClientShellState {
    pub(super) fn move_custom_menu_selection(&mut self, delta: isize) {
        let Some(ClientShellOverlay::CustomMenu(menu)) = self.overlay.as_mut() else {
            return;
        };
        let step = delta.signum();
        let mut index = menu.highlighted as isize + step;
        while let Some(item) = usize::try_from(index)
            .ok()
            .and_then(|index| menu.menu.items.get(index))
        {
            if !item.divider {
                menu.highlighted = index as usize;
                return;
            }
            index += step;
        }
    }

    pub(super) fn custom_menu_hotkey_index(&self, hotkey: char) -> Option<usize> {
        let Some(ClientShellOverlay::CustomMenu(menu)) = self.overlay.as_ref() else {
            return None;
        };
        let hotkey = hotkey.to_ascii_lowercase().to_string();
        menu.menu
            .items
            .iter()
            .position(|item| item.hotkey.as_deref() == Some(hotkey.as_str()))
    }

    pub(super) fn handle_custom_menu_endpoint_result(
        &mut self,
        binding_labels: &[String],
        result: Result<crate::api::schema::ResponseResult, ClientShellEndpointError>,
    ) -> bool {
        match result {
            Ok(crate::api::schema::ResponseResult::MenuList { menus }) => {
                let published = menus.into_iter().find(|candidate| {
                    !binding_labels.is_empty()
                        && binding_labels
                            .iter()
                            .all(|label| candidate.binding_labels.contains(label))
                });
                match published {
                    Some(menu) => {
                        // Another overlay may have opened while the request was in flight.
                        if self.overlay.is_none() {
                            self.overlay =
                                Some(ClientShellOverlay::CustomMenu(ClientCustomMenuOverlay {
                                    highlighted: menu
                                        .items
                                        .iter()
                                        .position(|item| !item.divider)
                                        .unwrap_or(0),
                                    menu,
                                }));
                        }
                    }
                    None => self.set_endpoint_error(
                        "custom menu is not available on this endpoint; reload configuration",
                    ),
                }
                true
            }
            Ok(_) => {
                self.set_endpoint_error("endpoint returned an unexpected menu list result");
                true
            }
            Err(_) => true,
        }
    }

    pub(super) fn activate_custom_menu_item(
        &mut self,
        index: usize,
        outcome: &mut ClientShellInput,
    ) {
        let Some(ClientShellOverlay::CustomMenu(menu)) = self.overlay.as_ref() else {
            return;
        };
        let Some(item) = menu.menu.items.get(index).filter(|item| !item.divider) else {
            return;
        };
        let command_id = item.command_id.clone();
        self.overlay = None;
        let Some(snapshot) = self.snapshot.as_deref() else {
            outcome.repaint = true;
            return;
        };
        let params = crate::api::schema::CommandInvokeParams {
            command_id,
            workspace_id: snapshot.focused_workspace_id.clone(),
            tab_id: snapshot.focused_tab_id.clone(),
            pane_id: snapshot.focused_pane_id.clone(),
            selection: None,
        };
        self.push_endpoint_method(crate::api::schema::Method::CommandInvoke(params), outcome);
        outcome.repaint = true;
    }
}
