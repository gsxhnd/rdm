use gpui::*;
use gpui_component::{Root, TitleBar};
use gpui_component_assets::Assets;

mod page;

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            let window_opt = WindowOptions {
                titlebar: Some(TitleBar::title_bar_options()),
                ..Default::default()
            };

            let _ = cx.open_window(window_opt, |window, cx| {
                let view = cx.new(|_| page::home::Home);
                cx.new(|cx| Root::new(view, window, cx))
            });
        })
        .detach();
    });
}
