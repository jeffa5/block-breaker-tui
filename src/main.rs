extern crate structopt;

mod opts;
mod widget;

use block_breaker::GameState;
use std::time::Duration;
use structopt::StructOpt;
use tui::backend::CrosstermBackend;
use tui::widgets::Widget;
use tui::Terminal;
use widget::GameWidget;

use crossterm::{InputEvent, KeyEvent};

fn render(
    mut f: tui::terminal::Frame<tui::backend::CrosstermBackend>,
    game_widget: &mut widget::GameWidget,
) {
    let size = f.size();
    game_widget.render(&mut f, size);
}

fn last_currently_available<T: Iterator>(iterator: &mut T) -> Option<T::Item> {
    let mut last = None;
    for value in iterator {
        last = Some(value)
    }
    last
}

fn main() -> Result<(), std::io::Error> {
    let opts = opts::Opts::from_args();

    let config = block_breaker::Config::new(
        opts.block_density,
        opts.block_strength,
        opts.bar_width,
        opts.ball_power,
    );

    let backend = CrosstermBackend::new();
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let terminal_size = terminal.size().unwrap();

    let mut game_state = GameState::new(terminal_size.width / 2, terminal_size.height, &config);

    if let Ok(_raw) = crossterm::RawScreen::into_raw_mode() {
        let input = crossterm::input();
        let mut reader = input.read_async();

        loop {
            let mut game_widget = GameWidget::new(&mut game_state);
            terminal.draw(|f| render(f, &mut game_widget))?;

            if let Some(key_event) = last_currently_available(&mut reader) {
                match key_event {
                    InputEvent::Keyboard(k) => match k {
                        KeyEvent::Ctrl('c') => break,
                        KeyEvent::Char('l') | KeyEvent::Right => game_state.bar_mut().move_right(),
                        KeyEvent::Char('h') | KeyEvent::Left => game_state.bar_mut().move_left(),
                        KeyEvent::Char(' ') => game_state.toggle_pause(),
                        KeyEvent::Char('q') => break,
                        KeyEvent::Char('r') => {
                            game_state = GameState::new(
                                terminal_size.width / 2,
                                terminal_size.height,
                                &config,
                            )
                        }
                        _ => continue,
                    },
                    _ => continue,
                }
            };

            game_state.tick();

            std::thread::sleep(Duration::from_millis(40));
        }
    }

    terminal.clear()?;
    Ok(())
}
