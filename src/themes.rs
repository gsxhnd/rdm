use gpui::App;
use gpui::SharedString;
use gpui_component::{Theme, ThemeRegistry};

pub fn init_theme(cx: &mut App) {
    let theme_path = match std::env::current_dir() {
        Ok(p) => p.join("themes"),
        Err(err) => {
            tracing::error!("Failed to init themes directory: {}", err);
            return;
        }
    };

    let theme_name = SharedString::from("Catppuccin Mocha");
    if let Err(err) = ThemeRegistry::watch_dir(theme_path, cx, move |cx| {
        let t = ThemeRegistry::global(cx);
        let ts = t.themes();
        for (k, _) in ts.iter() {
            println!("{:?}", k);
        }

        if let Some(theme) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
            Theme::global_mut(cx).apply_config(&theme);
        }
    }) {
        tracing::error!("Failed to watch themes directory: {}", err);
    }
}
