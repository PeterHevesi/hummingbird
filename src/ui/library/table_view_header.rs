use gpui::{prelude::FluentBuilder, *};

use crate::ui::{
    components::{
        icons::{GRID, GRID_INACTIVE, LIST, LIST_INACTIVE},
        nav_button::nav_button,
        table::table_data::TableData,
        table::{Table, TableViewMode},
        tooltip::build_tooltip,
    },
    theme::Theme,
};

use cntp_i18n::tr;

use super::navigation::NavigationView;

#[derive(IntoElement)]
pub struct TableViewHeader<T, C>
where
    T: TableData<C> + 'static,
    C: crate::ui::components::table::table_data::Column + 'static,
{
    navigation_view: Entity<NavigationView>,
    table: Entity<Table<T, C>>,
}

impl<T, C> TableViewHeader<T, C>
where
    T: TableData<C> + 'static,
    C: crate::ui::components::table::table_data::Column + 'static,
{
    pub fn new(navigation_view: Entity<NavigationView>, table: Entity<Table<T, C>>) -> Self {
        Self {
            navigation_view,
            table,
        }
    }
}

impl<T, C> RenderOnce for TableViewHeader<T, C>
where
    T: TableData<C> + 'static,
    C: crate::ui::components::table::table_data::Column + 'static,
{
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let view_mode = self.table.read(cx).get_view_mode(cx);
        let is_grid = view_mode == TableViewMode::Grid;

        div()
            .w_full()
            .flex()
            .flex_col()
            .child(self.navigation_view)
            .child(
                div()
                    .pt(px(10.0))
                    .pb(px(10.0))
                    .px(px(18.0))
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .line_height(px(26.0))
                            .font_weight(FontWeight::BOLD)
                            .text_size(px(26.0))
                            .pb(px(4.0))
                            .child(Table::<T, C>::get_table_name()),
                    )
                    .when(T::supports_grid_view(), |div_el| {
                        let table_for_list = self.table.clone();
                        let table_for_grid = self.table.clone();

                        div_el.child(
                            div()
                                .flex()
                                .gap_1()
                                .child(
                                    nav_button(
                                        "list_toggle",
                                        if !is_grid { LIST } else { LIST_INACTIVE },
                                    )
                                    .on_click(move |_, _, cx| {
                                        table_for_list.update(cx, |table, cx| {
                                            table.set_view_mode(TableViewMode::List, cx);
                                        });
                                    })
                                    .when(!is_grid, |this| {
                                        this.bg(theme.nav_button_pressed)
                                            .border_color(theme.nav_button_pressed_border)
                                    })
                                    .tooltip(build_tooltip(tr!("LIST_VIEW", "List View"))),
                                )
                                .child(
                                    nav_button(
                                        "grid_toggle",
                                        if is_grid { GRID } else { GRID_INACTIVE },
                                    )
                                    .on_click(move |_, _, cx| {
                                        table_for_grid.update(cx, |table, cx| {
                                            table.set_view_mode(TableViewMode::Grid, cx);
                                        });
                                    })
                                    .when(is_grid, |this| {
                                        this.bg(theme.nav_button_pressed)
                                            .border_color(theme.nav_button_pressed_border)
                                    })
                                    .tooltip(build_tooltip(tr!("GRID_VIEW", "Grid View"))),
                                ),
                        )
                    }),
            )
    }
}
