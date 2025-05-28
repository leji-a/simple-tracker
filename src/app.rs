use crate::model::WatchedEpisodes;
use crate::input::prompt_folder;
use crate::config::{save_config, load_config, Config};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::{collections::HashSet, io};

fn clear_screen() {
    #[cfg(windows)]
    { let _ = std::process::Command::new("cmd").args(["/C", "cls"]).status(); }
    #[cfg(not(windows))]
    { let _ = std::process::Command::new("clear").status(); }
}

pub fn run_history_menu(history: &HashSet<String>) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut config = load_config();
    let mut folders: Vec<String> = config.folder_history.iter().cloned().collect();
    folders.sort();
    let mut list_state = ListState::default();
    if !folders.is_empty() {
        list_state.select(Some(0));
    }

    let mut should_quit = false;
    let mut result = None;

    while !should_quit {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(4),
                    Constraint::Min(0),
                ].as_ref())
                .split(f.size());

            // Title and instructions
            let title = Paragraph::new(
                "Tracker - Select a folder or delete one with 'd', 'q' to quit"
            )
            .style(Style::default().add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            // Folder list
            let items: Vec<ListItem> = folders
                .iter()
                .map(|folder| ListItem::new(folder.clone()))
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Folder History"),
                )
                .highlight_style(Style::default().add_modifier(Modifier::BOLD));

            f.render_stateful_widget(list, chunks[1], &mut list_state);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    should_quit = true;
                }
                KeyCode::Char('d') => {
                    if let Some(i) = list_state.selected() {
                        if !folders.is_empty() {
                            let removed = folders.remove(i);
                            config.folder_history.remove(&removed);
                            save_config(&config).ok();
                            // Adjust selection
                            let new_len = folders.len();
                            if new_len == 0 {
                                list_state.select(None);
                            } else if i >= new_len {
                                list_state.select(Some(new_len - 1));
                            } else {
                                list_state.select(Some(i));
                            }
                        }
                    }
                }
                KeyCode::Down => {
                    if !folders.is_empty() {
                        let i = match list_state.selected() {
                            Some(i) => {
                                if i >= folders.len() - 1 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        list_state.select(Some(i));
                    }
                }
                KeyCode::Up => {
                    if !folders.is_empty() {
                        let i = match list_state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    folders.len() - 1
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        list_state.select(Some(i));
                    }
                }
                KeyCode::Enter => {
                    if let Some(i) = list_state.selected() {
                        if !folders.is_empty() {
                            result = Some(folders[i].clone());
                            should_quit = true;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    clear_screen();
    Ok(result)
}

pub fn run_app(
    episodes: &mut Vec<String>,
    watched: &mut WatchedEpisodes,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut list_state = ListState::default();
    if !episodes.is_empty() {
        list_state.select(Some(0));
    }

    loop {
        // Combine episodes from folder and watched.json (including missing ones)
        let mut all_episodes: Vec<String> = episodes.clone();
        for ep in watched.0.keys() {
            if !all_episodes.contains(ep) && ep != "watched.json" {
                all_episodes.push(ep.clone());
            }
        }
        all_episodes.sort();

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            let items: Vec<ListItem> = all_episodes
                .iter()
                .map(|ep| {
                    let watched_marker = if watched.0.get(ep).copied().unwrap_or(false) {
                        "[✔]"
                    } else {
                        "[ ]"
                    };
                    let missing_marker = if episodes.contains(ep) {
                        ""
                    } else {
                        " (missing)"
                    };
                    ListItem::new(format!("{} {}{}", watched_marker, ep, missing_marker))
                })
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                )
                .highlight_style(Style::default().add_modifier(Modifier::BOLD));

            f.render_stateful_widget(list, chunks[0], &mut list_state);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Down => {
                    let i = match list_state.selected() {
                        Some(i) => {
                            if i >= all_episodes.len() - 1 {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    list_state.select(Some(i));
                }
                KeyCode::Up => {
                    let i = match list_state.selected() {
                        Some(i) => {
                            if i == 0 {
                                all_episodes.len() - 1
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    list_state.select(Some(i));
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    if let Some(i) = list_state.selected() {
                        let ep = &all_episodes[i];
                        let entry = watched.0.entry(ep.clone()).or_insert(false);
                        *entry = !*entry;
                    }
                }
                _ => {}
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
