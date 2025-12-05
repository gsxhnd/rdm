use gpui::IntoElement;
use gpui::ParentElement;
use gpui::Render;
use gpui::div;
use gpui::px;

use gpui_component::resizable::{
    ResizablePanel, ResizablePanelEvent, ResizablePanelGroup, ResizableState, h_resizable,
    resizable_panel, v_resizable,
};

pub struct Panel;

impl Render for Panel {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        h_resizable("my-layout")
            .on_resize(|state, window, cx| {
                // Handle resize event
                // You can read the panel sizes from the state.
                let state = state.read(cx);
                let sizes = state.sizes();
            })
            .child(
                // Use resizable_panel() to create a sized panel.
                resizable_panel().size(px(200.)),
            )
            .child(
                // Or you can just add AnyElement without a size.
                div().child("123").into_any_element(),
            )
    }
}
