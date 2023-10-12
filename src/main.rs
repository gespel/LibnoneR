use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};

struct MainState;

impl MainState {
    fn new() -> Self {
        MainState
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::WHITE);

        // Zeichne eine Linie von (100, 100) nach (200, 200) in Rot
        let start = graphics::Point2::new(100.0, 100.0);
        let end = graphics::Point2::new(200.0, 200.0);
        let color = Color::new(1.0, 0.0, 0.0, 1.0);
        graphics::line(ctx, &[start, end], 2.0, color)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("rust_line_example", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = MainState::new();
    event::run(ctx, event_loop, state)
}
