use ggez::*;
use cgmath;
use ggez::graphics::DrawParam;
use cgmath::{Vector2, Point2};
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;

struct MainState {
    player1_coord: cgmath::Vector2<f32>,
    player2_coord: cgmath::Vector2<f32>,
    ball_coord: cgmath::Point2<f32>
}

impl MainState{
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let player1_coord = Vector2::new(100.0, 285.0);
        let player2_coord = Vector2::new(1170.0, 285.0);
        let ball_coord = Point2::new(640.0, 360.0);

        let s = MainState {
            player1_coord,
            player2_coord,
            ball_coord
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState{
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        if keyboard::is_key_pressed(ctx,KeyCode::W) && self.player1_coord.y >= 0.0 { self.player1_coord.y -= 10.0; }
        if keyboard::is_key_pressed(ctx, KeyCode::S) && self.player1_coord.y <= 570.0 { self.player1_coord.y += 10.0 }
        if keyboard::is_key_pressed(ctx,KeyCode::Up) && self.player2_coord.y >= 0.0 { self.player2_coord.y -= 10.0; }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) && self.player2_coord.y <= 570.0 { self.player2_coord.y += 10.0 }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        let line_char1 = graphics::Rect::new(self.player1_coord.x, self.player1_coord.y, 10.0,150.0);
        let line_char2 = graphics::Rect::new(self.player2_coord.x, self.player2_coord.y, 10.0, 150.0);
        let ball =
            graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), self.ball_coord, 10.0,2.0,graphics::WHITE)?;
        let r1 =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), line_char1, graphics::WHITE)?;

        let r2 =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), line_char2, graphics::WHITE)?;

        graphics::draw(ctx, &r1, DrawParam::default())?;
        graphics::draw(ctx, &r2, DrawParam::default())?;
        graphics::draw(ctx, &ball, DrawParam::default())?;
        graphics::present(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult{
    let cb = ggez::ContextBuilder::new("Game1","Brandon");
    let(ctx, events_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx).unwrap();

    graphics::set_drawable_size(ctx, 1280.0, 720.0).unwrap();
    graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, 1280.0, 720.0)).unwrap();
    graphics::set_window_title(ctx, "Rust Pong!");
    event::run(ctx,events_loop,state)
}