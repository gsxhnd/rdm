use std::any::Any;

use gpui::{AppContext, Element, Entity, Styled, div, px};
use gpui::{Context, Window};
use gpui::{ParentElement, Render};
use gpui_component::TitleBar;
use gpui_component::button::{Button, DropdownButton};
use gpui_component::checkbox::Checkbox;
use gpui_component::form::{Field, Form, field, h_form, v_form};
use gpui_component::group_box::{GroupBox, GroupBoxVariant, GroupBoxVariants as _};
use gpui_component::input::{Input, InputState};
use gpui_component::resizable::{h_resizable, resizable_panel};
use gpui_component::tooltip::Tooltip;
use gpui_component::tree::{TreeEntry, TreeItem, TreeState, tree};
use gpui_component::{IconName, StyledExt};
use gpui_component::{Side, sidebar::*};
use gpui_component::{h_flex, v_flex};

pub struct App {
    input_state: Entity<InputState>,
}

impl App {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx));

        Self { input_state: input }
    }
}

impl Render for App {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        v_flex()
            .child(init_titlebar())
            .child(h_flex().h_full().child(init_sidebar()))
            .child(Input::new(&self.input_state).cursor_context_menu())
            .child(format!("Value: {}", &self.input_state.read(cx).value()))
            .h_full()
    }
}

fn init_titlebar() -> impl gpui::IntoElement {
    TitleBar::new()
}

fn init_sidebar() -> impl gpui::IntoElement {
    h_resizable("multi-panel")
        .child(
            resizable_panel()
                .size(px(200.))
                .size_range(px(150.)..px(300.))
                .child("Left Panel"),
        )
        .child(resizable_panel().child("current"))
        .child(resizable_panel().size(px(250.)).child("Right Panel"))
}
