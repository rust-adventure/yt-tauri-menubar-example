#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    api::shell::open, AppHandle, CustomMenuItem, Manager,
    SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, SystemTraySubmenu,
};

const links: [(&str, &str, &str); 5] = [
    // social links
    ("open-social-netlify", "Netlify","https://app.netlify.com/teams/christopherbiscardi/overview"),
    ("open-social-youtube", "YouTube","https://www.youtube.com/@chrisbiscardi"),
    ("open-social-twitter", "Twitter","https://twitter.com/"),
    // github links
    ("open-github-rust-adventure", "Rust Adventure","https://github.com/rust-adventure"),
    ("open-github-bevy", "Bevy","https://github.com/bevyengine/bevy"),
];

fn main() {
    let sub_menu_social = {
        let mut menu = SystemTrayMenu::new();
        for (id, label, _url) in
            links.iter().filter(|(id, label, _url)| {
                id.starts_with("open-social")
            })
        {
            menu = menu.add_item(CustomMenuItem::new(
                id.to_string(),
                label.to_string(),
            ));
        }

        SystemTraySubmenu::new("Social", menu)
    };
    let sub_menu_github = {
        let mut menu = SystemTrayMenu::new();
        for (id, label, _url) in
            links.iter().filter(|(id, label, _url)| {
                id.starts_with("open-github")
            })
        {
            menu = menu.add_item(CustomMenuItem::new(
                id.to_string(),
                label.to_string(),
            ));
        }

        SystemTraySubmenu::new("GitHub", menu)
    };
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(
            "quit".to_string(),
            "Quit",
        ))
        .add_submenu(sub_menu_social)
        .add_submenu(sub_menu_github)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new(
            "visibility-toggle".to_string(),
            "Hide",
        ));

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(on_system_tray_event)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested {
                api, ..
            } => {
                api.prevent_exit();
            }
            _ => {}
        });
}

fn on_system_tray_event(
    app: &AppHandle,
    event: SystemTrayEvent,
) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle =
                app.tray_handle().get_item(&id);
            dbg!(&id);
            match id.as_str() {
                "visibility-toggle" => {
                    let window =
                        app.get_window("main").unwrap();
                    match window.is_visible() {
                        Ok(true) => {
                          window.hide().unwrap();
                          item_handle.set_title("Show").unwrap();
                        },
                        Ok(false) => {
                          window.show();
                          item_handle.set_title("Hide").unwrap();

                        },
                        Err(e) => unimplemented!("what kind of errors happen here?"),
                    }
                }
                "quit" => app.exit(0),
                s if s.starts_with("open-") => {
                    if let Some(link) = links
                        .iter()
                        .find(|(id, ..)| id == &s)
                    {
                        open(
                            &app.shell_scope(),
                            link.2,
                            None,
                        )
                        .unwrap();
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}
