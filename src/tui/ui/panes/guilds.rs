use super::*;

const COMPACT_ICON_WIDTH: u16 = 4;
const COMPACT_ICON_HEIGHT: u16 = 2;

pub(in crate::tui::ui) fn render_guilds(frame: &mut Frame, area: Rect, state: &DashboardState) {
    let focused = state.focus() == FocusPane::Guilds;
    let filter_query = state.guild_pane_filter_query();
    let block = panel_block("", focused);
    let inner = block.inner(area);
    frame.render_widget(block, area);
    let (list_area, filter_area) = split_pane_filter_area(inner, filter_query.is_some());
    let entries = state.visible_guild_pane_entries();
    let selected = state.focused_guild_selection();

    let items = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let is_selected = selected == Some(index);
            let is_active = state.is_active_guild_entry(entry);
            let (first, second) = match entry {
                GuildPaneEntry::DirectMessages => {
                    let style = selected_text_style(
                        is_selected,
                        active_text_style(
                            is_active,
                            theme::current().style(theme::HighlightGroup::Strong),
                        ),
                    );
                    let mut first = vec![selection_marker(is_selected), fallback("DM", style)];
                    if let Some(badge) = direct_message_badge(state, is_selected) {
                        first.push(badge);
                    }
                    (Line::from(first), Line::from(""))
                }
                GuildPaneEntry::FolderHeader { folder, collapsed } => {
                    let arrow = if *collapsed { "▶ " } else { "▼ " };
                    let icon = if *collapsed { "📁" } else { "📂" };
                    let style = folder_style(folder.color);
                    (
                        Line::from(vec![
                            selection_marker(is_selected),
                            Span::styled(
                                arrow,
                                selected_discord_text_style(is_selected, style, folder.color),
                            ),
                            Span::styled(
                                icon,
                                selected_discord_text_style(is_selected, style, folder.color),
                            ),
                        ]),
                        Line::from(""),
                    )
                }
                GuildPaneEntry::Guild {
                    state: guild,
                    branch,
                } => {
                    let mut style = active_text_style(is_active, Style::default());
                    if state.guild_notification_muted(guild.id) {
                        style = theme::current().apply(theme::HighlightGroup::Muted, style);
                    }
                    let style = selected_text_style(is_selected, style);
                    let badge = guild_badge(state, guild, is_active, is_selected);
                    let mut first = vec![
                        selection_marker(is_selected),
                        Span::styled(
                            branch.prefix(),
                            theme::current().style(theme::HighlightGroup::Decoration),
                        ),
                        fallback(&initial(&guild.name), style),
                    ];
                    if let Some(badge) = badge {
                        first.push(badge);
                    }
                    (
                        Line::from(first),
                        Line::from(vec![
                            selection_marker(false),
                            Span::raw(" ".repeat(COMPACT_ICON_WIDTH as usize)),
                        ]),
                    )
                }
            };
            ListItem::new(vec![
                selected_row_line(first, is_selected),
                selected_row_line(second, is_selected),
            ])
            .style(selected_row_style(is_selected))
        })
        .collect::<Vec<_>>();
    frame.render_widget(List::new(items), list_area);

    render_pane_filter_bar_with_cursor(
        frame,
        filter_area,
        filter_query,
        state.guild_pane_filter_cursor(),
        focused,
    );
    render_vertical_scrollbar(
        frame,
        list_area,
        state.guild_scroll(),
        list_area.height as usize / GUILD_PANE_ENTRY_HEIGHT,
        state.guild_pane_filtered_entries().len(),
    );
}

fn guild_icon_area(
    list_area: Rect,
    index: usize,
    branch: GuildBranch,
    marker_width: u16,
    horizontal_scroll: usize,
) -> Option<Rect> {
    let x = list_area
        .x
        .saturating_add(marker_width)
        .saturating_add(branch.prefix().width() as u16)
        .saturating_sub(u16::try_from(horizontal_scroll).unwrap_or(u16::MAX));
    let y = list_area.y.saturating_add(
        u16::try_from(index)
            .unwrap_or(u16::MAX)
            .saturating_mul(GUILD_PANE_ENTRY_HEIGHT as u16),
    );
    let right = x.saturating_add(COMPACT_ICON_WIDTH);
    let list_right = list_area.x.saturating_add(list_area.width);
    let clipped_x = x.max(list_area.x);
    let clipped_right = right.min(list_right);
    (clipped_x < clipped_right).then(|| {
        Rect::new(
            clipped_x,
            y,
            clipped_right.saturating_sub(clipped_x),
            COMPACT_ICON_HEIGHT,
        )
    })
}

fn initial(name: &str) -> String {
    name.chars()
        .next()
        .map(|c| c.to_uppercase().collect())
        .unwrap_or_else(|| "?".to_owned())
}

fn fallback(label: &str, style: Style) -> Span<'static> {
    let width = label.width();
    let left = (COMPACT_ICON_WIDTH as usize).saturating_sub(width) / 2;
    Span::styled(
        format!(
            "{}{}{}",
            " ".repeat(left),
            label,
            " ".repeat((COMPACT_ICON_WIDTH as usize).saturating_sub(left + width))
        ),
        style,
    )
}

fn guild_badge(
    state: &DashboardState,
    guild: &crate::discord::GuildState,
    active: bool,
    selected: bool,
) -> Option<Span<'static>> {
    let unread = state.sidebar_guild_unread(guild.id);
    let (badge, _) = if active {
        channel_unread_decoration(unread, Style::default(), false)
    } else if unread == ChannelUnreadState::Seen {
        (None, Style::default())
    } else {
        channel_unread_decoration(unread, Style::default(), false)
    };
    badge.map(|badge| selected_text_span(selected, badge))
}

fn direct_message_badge(state: &DashboardState, selected: bool) -> Option<Span<'static>> {
    let count = state.direct_message_unread_count();
    (count > 0).then(|| {
        let count = u32::try_from(count).unwrap_or(u32::MAX);
        selected_text_span(
            selected,
            notification_count_badge(ChannelUnreadState::Notified(count)),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guild_icon_area_is_four_by_two_and_clipped_to_list() {
        let list = Rect::new(10, 4, 12, 8);
        assert_eq!(
            guild_icon_area(list, 1, GuildBranch::None, 2, 0),
            Some(Rect::new(12, 6, 4, 2))
        );
        assert_eq!(
            guild_icon_area(list, 1, GuildBranch::None, 2, 5),
            Some(Rect::new(10, 6, 1, 2))
        );
        assert_eq!(guild_icon_area(list, 1, GuildBranch::None, 2, 7), None);
    }
}
