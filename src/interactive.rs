use crate::{filters::{Filter}, prelude::*, state::State};
use crossterm::{
    cursor::{Hide, MoveTo, Show}, 
    event::{read, KeyCode, KeyModifiers}, 
    execute, 
    style::{Color, Print, SetForegroundColor}, 
    terminal::{self, Clear, ClearType}
};

pub fn run(filter: Box<dyn Filter>, &count_choices: &u8) -> Result<(), Box<dyn Error>> {
    let mut state = State::load(filter, &count_choices)?;

    let stdout = std::io::stdout();
    let mut tty = File::create("/dev/tty")?;

    terminal::enable_raw_mode()?;
    execute!(tty, Hide)?;

    loop {
        execute!(tty, Clear(ClearType::All), MoveTo(0, 0))?;
        execute!(
            tty, 
            SetForegroundColor(Color::Yellow), 
            Print(format!("> {}\n", state.current_cmd_mask)), 
            SetForegroundColor(Color::Reset)
        )?;

        if let Some(indices) = &state.filtered_indices_cmds {
            for (i, &idx) in indices.iter().enumerate() {
                if let Some(cmd) = state.cmds.get(idx) {
                    execute!(tty, MoveTo(0, 1  + i as u16))?;
                    if i == state.selected_index_cmd {
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
                    state.current_cmd_mask.push(ch);
                    state.search();
                },
                KeyCode::Backspace => {
                    state.current_cmd_mask.pop();
                    state.search();
                },
                KeyCode::Up => {
                    if state.filtered_indices_cmds.is_some() && state.selected_index_cmd > 0 {
                        state.selected_index_cmd -= 1;
                    }
                },
                KeyCode::Down => {
                    if let Some(indices) = &state.filtered_indices_cmds {
                        if state.selected_index_cmd < indices.len().saturating_sub(1) {
                            state.selected_index_cmd += 1;
                        }
                    }
                },
                KeyCode::Enter => {
                    if let Some(indices) = &state.filtered_indices_cmds {
                        if let Some(&selected_cmd_idx) = indices.get(state.selected_index_cmd) {
                            if let Some(selected_cmd) = state.cmds.get(selected_cmd_idx) {
                                execute!(&stdout, Print(format!("bash -ci \"{selected_cmd}\"")))?;
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