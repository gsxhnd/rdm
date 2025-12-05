use gpui::{AppContext, Entity, Styled};
use gpui::{ParentElement, Render};
use gpui_component::TitleBar;
use gpui_component::button::Button;
use gpui_component::checkbox::Checkbox;
use gpui_component::group_box::{GroupBox, GroupBoxVariant, GroupBoxVariants as _};
use gpui_component::select::{
    SearchableVec, Select, SelectDelegate, SelectEvent, SelectGroup, SelectItem, SelectState,
};
use gpui_component::{IconName, StyledExt};
use gpui_component::{Side, sidebar::*};
use gpui_component::{h_flex, v_flex};

pub struct App;

impl Render for App {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let languages: SearchableVec<&str> = SearchableVec::new(vec![
            "Rust".into(),
            "TypeScript".into(),
            "Go".into(),
            "Python".into(),
            "JavaScript".into(),
        ]);
        let state = cx.new(|cx| SelectState::new(languages, None, window, cx));
        // let select = Select::new(&state).placeholder("Select language...");
        v_flex()
            .child(init_titlebar())
            .child(
                h_flex().h_full().child(init_sidebar()).child(
                    Select::new(&state)
                        .placeholder("Select language...")
                        .search_placeholder("sda")
                        .w_128(),
                ),
            )
            .h_full()
    }
}

fn init_titlebar() -> impl gpui::IntoElement {
    TitleBar::new()
}

fn init_sidebar() -> impl gpui::IntoElement {
    Sidebar::new(Side::Left)
        .collapsed(true)
        .header(SidebarHeader::new().child("My Application"))
        .child(
            SidebarGroup::new("Navigation").child(
                SidebarMenu::new()
                    .child(
                        SidebarMenuItem::new("Dashboard")
                            .icon(IconName::LayoutDashboard)
                            .on_click(|_, _, _| println!("Dashboard clicked")),
                    )
                    .child(
                        SidebarMenuItem::new("Settings")
                            .icon(IconName::Settings)
                            .on_click(|_, _, _| println!("Settings clicked")),
                    ),
            ),
        )
        .footer(
            SidebarFooter::new().child(
                SidebarMenuItem::new("Settings")
                    .icon(IconName::Settings)
                    .on_click(|_, _, _| println!("Settings clicked")),
            ),
        )
        .h_full()
}
