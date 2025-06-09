use chrono::Local;
use tui::Frame;
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::{Modifier, Style};
use tui::text::{Line, Span};
use tui::widgets::Paragraph;

use crate::ui::{AllTasksPage, DeleteTaskPage, InputMode, TaskPage, UIPage};

pub fn render_bottom_bar(
    f: &mut Frame,
    area: Rect,
    all_tasks_page: &AllTasksPage,
    task_page: &TaskPage,
    delete_task_page: &Option<DeleteTaskPage>,
    current_page: &UIPage,
    chunks: &[Rect],
) {
    let colors = &all_tasks_page.app.borrow().settings.colors;
    let (mode_str, mode_color) = match current_page {
        UIPage::NewTask | UIPage::EditTask => match task_page.input_mode {
            InputMode::Insert => ("INSERT", colors.insert_mode_color),
            InputMode::Normal => ("NORMAL", colors.normal_mode_color),
            InputMode::Visual => ("VISUAL", colors.visual_mode_color),
            InputMode::Command => ("COMMAND", colors.command_mode_color),
        },
        UIPage::DeleteTask => match delete_task_page {
            Some(dtp) => match dtp.input_mode {
                InputMode::Insert => ("INSERT", colors.insert_mode_color),
                InputMode::Normal => ("NORMAL", colors.normal_mode_color),
                InputMode::Visual => ("VISUAL", colors.visual_mode_color),
                InputMode::Command => ("COMMAND", colors.command_mode_color),
            },
            None => ("NORMAL", colors.normal_mode_color),
        },
        UIPage::AllTasks => ("NORMAL", colors.normal_mode_color),
    };

    let left_margin = " ".repeat(chunks[0].x as usize);
    let powerline_r = ""; // Use '>' as fallback if not supported
    let powerline_l= ""; // Use '>' as fallback if not supported
    let now = Local::now().format("%H:%M").to_string();

    // Colors
    let mode_color = mode_color;
    let neutral_dark = colors.neutral_dark;
    let neutral_light = colors.neutral_light;
    let fg_dark = colors.foreground_dark;
    let fg_light = colors.foreground_light;

    // Sections (left)
    let mode_section = Span::styled(
        format!(" {} ", mode_str),
        Style::default()
            .fg(fg_dark)
            .bg(mode_color)
            .add_modifier(Modifier::BOLD),
    );
    let mode_arrow = Span::styled(powerline_r, Style::default().fg(mode_color).bg(neutral_light));
    let branch_section = Span::styled(
        " main ",
        Style::default()
            .fg(fg_light)
            .bg(neutral_light)
            .add_modifier(Modifier::BOLD),
    );
    let branch_arrow = Span::styled(powerline_r, Style::default().fg(neutral_light).bg(neutral_dark));
    let left_line = Line::from(vec![
        Span::raw(left_margin),
        mode_section,
        mode_arrow,
        branch_section,
        branch_arrow,
    ]);

    // Section (right)
    let time_section = Span::styled(
        format!(" {} ", now),
        Style::default().fg(fg_dark).bg(mode_color),
    );
    let time_arrow = Span::styled(powerline_l, Style::default().fg(mode_color).bg(neutral_dark));
    let right_line = Line::from(vec![time_arrow, time_section]);

    // Layout: left and right chunks
    let bar_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(right_line.width() as u16),
        ])
        .split(area);

    let left_paragraph = Paragraph::new(left_line).alignment(tui::layout::Alignment::Left);
    let right_paragraph = Paragraph::new(right_line).alignment(tui::layout::Alignment::Right);
    f.render_widget(left_paragraph, bar_chunks[0]);
    f.render_widget(right_paragraph, bar_chunks[1]);
}
