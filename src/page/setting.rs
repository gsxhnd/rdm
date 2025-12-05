use gpui::{App, Render};
use gpui_component::setting::{SettingField, SettingGroup, SettingItem, SettingPage, Settings};
pub struct Setting;

impl Render for Setting {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        Settings::new("my-settings").pages(vec![SettingPage::new("General").group(
            SettingGroup::new().title("Basic Options").item(
                SettingItem::new(
                    "Enable Feature",
                    SettingField::switch(
                        |cx: &App| true,
                        |val: bool, cx: &mut App| {
                            println!("Feature enabled: {}", val);
                        },
                    ),
                ),
                // SettingItem::new("", SettingField::),
            ),
        )])
    }
}
