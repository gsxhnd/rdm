use gpui::ParentElement;
use gpui::Render;
use gpui::Styled;
// use gpui::div;
use gpui_component::StyledExt;
use gpui_component::TitleBar;
use gpui_component::button::Button;
use gpui_component::h_flex;
use gpui_component::v_flex;

pub struct Home;

impl Render for Home {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        v_flex().child(
            TitleBar::new().child(
                h_flex()
                    .w_full()
                    .pr_2()
                    .justify_between()
                    .child("App with Custom title bar")
                    .child("Right Item"),
            ),
        )
        // div()
        //     .v_flex()
        //     .gap_2()
        //     .size_full()
        //     .items_center()
        //     .justify_center()
        //     .child(Button::new("btn").label("Click Me"))
    }
}
