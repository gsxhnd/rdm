use gpui::{AppContext, Application, Window, WindowOptions};
use gpui::{Size, px};
use gpui_component::{Root, TitleBar, window_border};
use gpui_component_assets::Assets;

mod page;

fn main() {
    tracing_subscriber::fmt::init();

    let app = Application::new().with_assets(Assets);
    tracing::error!("preference dir: {:?}", dirs::preference_dir());

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            let window_opt = WindowOptions {
                titlebar: Some(TitleBar::title_bar_options()),
                window_decorations: Some(gpui::WindowDecorations::Client),
                window_min_size: Some(Size {
                    width: px(640.),
                    height: px(480.),
                }),
                ..Default::default()
            };

            let _ = cx.open_window(window_opt, |window, cx| {
                let view = cx.new(|_| page::home::Home);
                // let view = cx.new(|_| page::app::App);
                cx.new(|cx| Root::new(view, window, cx))
            });
        })
        .detach();
    });
}
