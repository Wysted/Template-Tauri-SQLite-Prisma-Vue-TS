use tauri::{ Menu, MenuItem, Submenu };

pub fn get_menu() -> Menu {
    let example = Submenu::new("Example", Menu::new().add_native_item(MenuItem::Quit));

    let edit = Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::SelectAll)
    );

    let window = Submenu::new(
        "Window",
        Menu::new().add_native_item(MenuItem::EnterFullScreen).add_native_item(MenuItem::Minimize)
    );

    let menu = Menu::new().add_submenu(example).add_submenu(edit).add_submenu(window);

    menu
}
