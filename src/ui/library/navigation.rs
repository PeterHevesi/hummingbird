use gpui::{prelude::FluentBuilder, *};
use tracing::debug;

use crate::{
    library::db::{AlbumMethod, LibraryAccess},
    ui::components::{
        icons::{ARROW_LEFT, ARROW_RIGHT},
        nav_button::nav_button,
        table::table_data::TABLE_MAX_WIDTH,
    },
};

use super::{NavigationHistory, ViewSwitchMessage};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NavigationDisplayMode {
    Visible,
    Spacer,
}

impl NavigationDisplayMode {
    pub(super) fn shows_buttons(self) -> bool {
        matches!(self, Self::Visible)
    }
}

pub(super) struct NavigationView {
    view_switcher_model: Entity<NavigationHistory>,
    current_message: ViewSwitchMessage,
    description: Option<SharedString>,
    display_mode: NavigationDisplayMode,
}

impl NavigationView {
    pub(super) fn new(
        cx: &mut App,
        view_switcher_model: Entity<NavigationHistory>,
        display_mode: NavigationDisplayMode,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let current_message = view_switcher_model.read(cx).current();

            cx.observe(&view_switcher_model, |this: &mut NavigationView, m, cx| {
                debug!("{:#?}", m.read(cx));

                this.current_message = m.read(cx).current();

                this.description = match this.current_message {
                    ViewSwitchMessage::Release(id, _) => cx
                        .get_album_by_id(id, AlbumMethod::Metadata)
                        .ok()
                        .map(|v| SharedString::from(v.title.clone())),
                    ViewSwitchMessage::Artist(id) => cx
                        .get_artist_by_id(id)
                        .ok()
                        .and_then(|v| v.name.as_ref().map(|n| n.0.clone())),
                    _ => None,
                }
            })
            .detach();

            Self {
                view_switcher_model,
                current_message,
                description: None,
                display_mode,
            }
        })
    }

    pub(super) fn set_display_mode(
        &mut self,
        display_mode: NavigationDisplayMode,
        cx: &mut Context<Self>,
    ) {
        if self.display_mode != display_mode {
            self.display_mode = display_mode;
            cx.notify();
        }
    }

    pub(super) fn height() -> Pixels {
        px(38.0)
    }
}

impl Render for NavigationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.display_mode.shows_buttons() {
            return div().w_full().h(Self::height());
        }

        let can_go_back = self.view_switcher_model.read(cx).can_go_back();
        let can_go_forward = self.view_switcher_model.read(cx).can_go_forward();

        let settings = cx
            .global::<crate::settings::SettingsGlobal>()
            .model
            .read(cx);
        let full_width = settings.interface.effective_full_width();

        div().flex().child(
            div()
                .flex()
                .gap(px(4.0))
                .w_full()
                .when(!full_width, |this: Div| this.max_w(px(TABLE_MAX_WIDTH)))
                .mr_auto()
                .pl(px(10.0))
                .pt(px(10.0))
                .child(
                    nav_button("back", ARROW_LEFT)
                        .disabled(!can_go_back)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.view_switcher_model.update(cx, |_, cx| {
                                cx.emit(ViewSwitchMessage::Back);
                            })
                        })),
                )
                .child(
                    nav_button("forward", ARROW_RIGHT)
                        .disabled(!can_go_forward)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.view_switcher_model.update(cx, |_, cx| {
                                cx.emit(ViewSwitchMessage::Forward);
                            })
                        })),
                ),
        )
    }
}
