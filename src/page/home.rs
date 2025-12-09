use gpui::Context;
use gpui::Entity;
use gpui::ParentElement;
use gpui::Pixels;
use gpui::Render;
use gpui::Styled;
use gpui::px;

use gpui_component::TitleBar;
use gpui_component::button::Button;
use gpui_component::checkbox::Checkbox;

use gpui_component::h_flex;
use gpui_component::resizable::{
    ResizablePanel, ResizablePanelEvent, ResizablePanelGroup, ResizableState, h_resizable,
    resizable_panel, v_resizable,
};
use gpui_component::v_flex;

pub struct Home;

impl Render for Home {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        v_flex()
            .child(
                TitleBar::new().child(
                    h_flex()
                        .w_full()
                        .pr_2()
                        .justify_between()
                        .child("App with Custom title bar")
                        .child("Right Item"),
                ),
            )
            .child(v_flex().gap_2().items_center().justify_center().child(
                Button::new("btn").label("Click Me").on_click(|x, w, a| {
                    let size = w.rem_size();
                    w.toggle_fullscreen();
                    tracing::info!("size: {}", size);
                }),
            ))
    }
}
