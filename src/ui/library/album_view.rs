use std::rc::Rc;

use gpui::{prelude::FluentBuilder, *};

use crate::{
    library::{
        scan::ScanEvent,
        types::{Album, table::AlbumColumn},
    },
    ui::{
        components::table::{Table, TableEvent, table_data::TABLE_MAX_WIDTH},
        library::{
            NavigationDisplayMode, context_menus::AlbumContextMenuContext,
            table_view_header::TableViewHeader,
        },
        models::Models,
    },
};

use super::{NavigationHistory, ViewSwitchMessage};

#[derive(Clone)]
pub struct AlbumView {
    table: Entity<Table<Album, AlbumColumn>>,
    table_view_header: Entity<TableViewHeader<Album, AlbumColumn>>,
}

impl AlbumView {
    pub(super) fn new(
        cx: &mut App,
        view_switch_model: Entity<NavigationHistory>,
        navigation_mode: NavigationDisplayMode,
        initial_scroll_offset: Option<f32>,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let state = cx.global::<Models>().scan_state.clone();

            let table_settings = cx.global::<Models>().table_settings.clone();
            let initial_settings = table_settings
                .read(cx)
                .get(Table::<Album, AlbumColumn>::get_table_name().as_str())
                .cloned();

            let handler_model = view_switch_model.clone();
            let handler = Rc::new(move |cx: &mut App, id: &(u32, String)| {
                handler_model.update(cx, |_, cx| {
                    cx.emit(ViewSwitchMessage::Release(id.0 as i64, None))
                })
            });

            let table = Table::new(
                cx,
                Some(handler),
                AlbumContextMenuContext::default(),
                initial_scroll_offset,
                initial_settings.as_ref(),
            );

            let table_clone = table.clone();

            cx.observe(&state, move |_: &mut AlbumView, e, cx| {
                let value = e.read(cx);
                match value {
                    ScanEvent::ScanCompleteIdle => {
                        table_clone.update(cx, |_, cx| cx.emit(TableEvent::NewRows));
                    }
                    ScanEvent::ScanProgress { current, .. } => {
                        if current % 100 == 0 {
                            table_clone.update(cx, |_, cx| cx.emit(TableEvent::NewRows));
                        }
                    }
                    _ => {}
                }
            })
            .detach();

            AlbumView {
                table_view_header: TableViewHeader::new(
                    cx,
                    view_switch_model.clone(),
                    navigation_mode,
                    table.clone(),
                ),
                table,
            }
        })
    }

    pub fn get_scroll_offset(&self, cx: &App) -> f32 {
        self.table.read(cx).get_scroll_offset(cx)
    }

    pub fn set_navigation_display_mode(
        &mut self,
        navigation_display_mode: NavigationDisplayMode,
        cx: &mut Context<Self>,
    ) {
        self.table_view_header.update(cx, |header, cx| {
            header.set_navigation_display_mode(navigation_display_mode, cx);
        });
    }
}

impl Render for AlbumView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = cx
            .global::<crate::settings::SettingsGlobal>()
            .model
            .read(cx);
        let full_width = settings.interface.effective_full_width();

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .when(!full_width, |this: Div| this.max_w(px(TABLE_MAX_WIDTH)))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .h_full()
                    .child(self.table_view_header.clone())
                    .child(self.table.clone()),
            )
    }
}
