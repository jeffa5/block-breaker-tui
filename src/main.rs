use block_breaker;

use std::time::Duration;

use tui::backend::CrosstermBackend;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::Widget;
use tui::Terminal;

use crossterm::{InputEvent, KeyEvent};

struct BlockBreaker<'a> {
    game_state: &'a mut block_breaker::GameState,
}

impl<'a> BlockBreaker<'a> {
    fn new(game_state: &'a mut block_breaker::GameState) -> BlockBreaker {
        BlockBreaker { game_state }
    }
}

fn draw_centered_text(buf: &mut Buffer, width: u16, y: u16, text: &str, style: Style) {
    buf.set_string((width / 2) - (text.len() as u16 / 2), y, text, style)
}

static PAUSED_STRING: &str = "Paused, press <space> to unpause";
static HELP_STRING: &str = "r: restart  <space>: pause  h/<-: left  l/->: right";
static GAME_OVER_STRING: &str = "GAME OVER, r to reset";

impl<'a> Widget for BlockBreaker<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        // due to being in a terminal characters are not squares, compensate by using double space characters

        self.background(area, buf, Color::Black);
        self.game_state
            .update_dimensions(area.width / 2, area.height);

        // draw the bar
        let player_bar = self.game_state.bar();
        let mut bar_string = String::with_capacity((player_bar.width()) as usize);
        for _ in 0..player_bar.width() {
            bar_string.push_str("  ")
        }
        buf.set_string(
            player_bar.x() * 2,
            player_bar.y(),
            bar_string,
            Style::default().bg(Color::White),
        );

        // draw the ball
        let ball = self.game_state.ball();
        let ball_string = "  ";
        buf.set_string(
            ball.x() * 2,
            ball.y(),
            ball_string,
            Style::default().bg(Color::Red),
        );

        // draw the blocks
        let blocks = self.game_state.blocks();
        let block_string = "    ";
        for block in blocks {
            buf.set_string(
                block.x() * 2,
                block.y(),
                block_string,
                Style::default().bg(Color::Green),
            )
        }

        // if game over, then show text of how to play again
        if self.game_state.game_over() {
            draw_centered_text(
                buf,
                area.width,
                area.height - 1,
                GAME_OVER_STRING,
                Style::default().fg(Color::Red),
            )
        } else if self.game_state.is_paused() {
            draw_centered_text(
                buf,
                area.width,
                area.height - 1,
                PAUSED_STRING,
                Style::default(),
            )
        } else {
            draw_centered_text(
                buf,
                area.width,
                area.height - 1,
                HELP_STRING,
                Style::default(),
            )
        }
    }
}

fn render(
    mut f: tui::terminal::Frame<tui::backend::CrosstermBackend>,
    blockbreaker: &mut BlockBreaker,
) {
    let size = f.size();
    blockbreaker.render(&mut f, size);
}

fn main() -> Result<(), std::io::Error> {
    let backend = CrosstermBackend::new();
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let terminal_size = terminal.size().unwrap();

    let block_density = 0.2;

    let mut game_state =
        block_breaker::GameState::new(terminal_size.width / 2, terminal_size.height, block_density);

    if let Ok(_raw) = crossterm::RawScreen::into_raw_mode() {
        let input = crossterm::input();
        let mut reader = input.read_async();

        loop {
            let mut block_breaker = BlockBreaker::new(&mut game_state);
            terminal.draw(|f| render(f, &mut block_breaker))?;

            if let Some(key_event) = reader.next() {
                match key_event {
                    InputEvent::Keyboard(k) => match k {
                        KeyEvent::Ctrl('c') => break,
                        KeyEvent::Char('l') | KeyEvent::Right => game_state.bar_mut().move_right(),
                        KeyEvent::Char('h') | KeyEvent::Left => game_state.bar_mut().move_left(),
                        KeyEvent::Char(' ') => game_state.toggle_pause(),
                        KeyEvent::Char('r') => {
                            game_state = block_breaker::GameState::new(
                                terminal_size.width / 2,
                                terminal_size.height,
                                block_density,
                            )
                        }
                        _ => continue,
                    },
                    _ => continue,
                }
            };

            game_state.tick();

            std::thread::sleep(Duration::from_millis(20));
        }
    }

    terminal.clear()?;
    Ok(())
}
