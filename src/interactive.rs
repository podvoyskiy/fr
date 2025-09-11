use crate::prelude::{AppError, Filtering, CommandHistory};
use std::fs::File;
use crossterm::{
    cursor::{Hide, MoveTo, Show}, 
    event::{read, KeyCode, KeyModifiers}, 
    execute, 
    style::{Color, Print, SetForegroundColor}, 
    terminal::{self, Clear, ClearType}
};

pub fn run(filter: Box<dyn Filtering>, &max_results: &u8) -> Result<(), AppError> {
    let mut history = CommandHistory::load(filter, &max_results)?;

    let stdout = std::io::stdout();
    let mut tty = File::create("/dev/tty")?;

    terminal::enable_raw_mode()?;
    execute!(tty, Hide)?;

    loop {
        execute!(tty, Clear(ClearType::All), MoveTo(0, 0))?;
        execute!(
            tty, 
            SetForegroundColor(Color::Yellow), 
            Print(format!("> {}\n", history.search_query)), 
            SetForegroundColor(Color::Reset)
        )?;

        if let Some(indices) = &history.filtered_indices {
            for (i, &idx) in indices.iter().enumerate() {
                if let Some(cmd) = history.commands.get(idx) {
                    execute!(tty, MoveTo(0, 1  + i as u16))?;
                    if i == history.selected_index {
                        execute!(
                            tty,
                            SetForegroundColor(Color::Cyan),
                            Print(format!("> {cmd}")),
                            SetForegroundColor(Color::Reset),
                        )?;
                    } else {
                        execute!(tty, Print(cmd))?;
                    }
                }
            }
        }
        
        if let crossterm::event::Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    break;
                },
                KeyCode::Esc => break,
                KeyCode::Char(ch) => {
                    history.search_query.push(ch);
                    history.search();
                },
                KeyCode::Backspace => {
                    history.search_query.pop();
                    history.search();
                },
                KeyCode::Up => {
                    if history.filtered_indices.is_some() && history.selected_index > 0 {
                        history.selected_index -= 1;
                    }
                },
                KeyCode::Down => {
                    if let Some(indices) = &history.filtered_indices {
                        if history.selected_index < indices.len().saturating_sub(1) {
                            history.selected_index += 1;
                        }
                    }
                },
                KeyCode::Enter => {
                    if let Some(indices) = &history.filtered_indices {
                        if let Some(&selected_cmd_idx) = indices.get(history.selected_index) {
                            if let Some(selected_cmd) = history.commands.get(selected_cmd_idx) {
                                execute!(&stdout, Print(format!("history -s \"{selected_cmd}\"\n{selected_cmd}\n")))?;
                                break;
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }

    execute!(tty, Show, Clear(ClearType::All), MoveTo(0, 0))?;
    terminal::disable_raw_mode()?;

    Ok(())
}