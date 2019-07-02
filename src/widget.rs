use block_breaker::GameState;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::Widget;

pub struct GameWidget<'a> {
    game_state: &'a mut GameState,
}

impl<'a> GameWidget<'a> {
    pub fn new(game_state: &'a mut GameState) -> GameWidget {
        GameWidget { game_state }
    }
}

fn draw_centered_text(buf: &mut Buffer, width: u16, y: u16, text: &str, style: Style) {
    buf.set_string((width / 2) - (text.len() as u16 / 2), y, text, style)
}

static PAUSED_STRING: &str = "PAUSED";
static HELP_STRING: &str = "q: quit r: restart  <space>: pause  h/<-: left  l/->: right";
static GAME_OVER_STRING: &str = "GAME OVER";
static SQUARE: &str = "  ";

impl<'a> Widget for GameWidget<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        // due to being in a terminal characters are not squares, compensate by using double space characters

        self.background(area, buf, Color::Black);
        self.game_state
            .update_dimensions(area.width / 2, area.height);

        // draw the bar
        let player_bar = self.game_state.bar();
        let mut bar_string = String::with_capacity((player_bar.width()) as usize);
        for _ in 0..player_bar.width() {
            bar_string.push_str(SQUARE)
        }
        buf.set_string(
            player_bar.x() * 2,
            player_bar.y(),
            bar_string,
            Style::default().bg(Color::White),
        );

        // draw the ball
        let ball = self.game_state.ball();
        let ball_string = SQUARE;
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
                area.height - 10,
                GAME_OVER_STRING,
                Style::default().fg(Color::Red),
            )
        } else if self.game_state.is_paused() {
            draw_centered_text(
                buf,
                area.width,
                area.height - 10,
                PAUSED_STRING,
                Style::default().fg(Color::Blue),
            )
        }
        draw_centered_text(
            buf,
            area.width,
            area.height - 1,
            HELP_STRING,
            Style::default(),
        )
    }
}
