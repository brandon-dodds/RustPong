use cgmath;
use cgmath::{Point2, Vector2};
use ggez::graphics::DrawParam;
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::*;

//The main state of the program (players, balls).
struct MainState {
    player1_coord: Vector2<f32>,
    player2_coord: Vector2<f32>,
    ball_coord: Point2<f32>,
}
//Implements the main state. New is created on creation of the game state.
impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let player1_coord = Vector2::new(100.0, 285.0);
        let player2_coord = Vector2::new(1170.0, 285.0);
        let ball_coord = Point2::new(640.0, 360.0);

        let s = MainState {
            player1_coord,
            player2_coord,
            ball_coord,
        };

        Ok(s)
    }
    //Resetting the ball position/Handling the physics of ball.
    fn ball_update_position(&mut self, player_val: i8) {
        if player_val == 1 {
            let paddle_middle = (self.player1_coord.y + self.player1_coord.y + 150.0) / 2.0;
            let diff = (paddle_middle - self.ball_coord.y).abs();
            println!("{}", diff);
            self.ball_coord = Point2::new(640.0, 360.0)
        } else {
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //Keyboard control code.
        if keyboard::is_key_pressed(ctx, KeyCode::W) && self.player1_coord.y >= 0.0 {
            self.player1_coord.y -= 10.0;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S) && self.player1_coord.y <= 570.0 {
            self.player1_coord.y += 10.0
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Up) && self.player2_coord.y >= 0.0 {
            self.player2_coord.y -= 10.0;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) && self.player2_coord.y <= 570.0 {
            self.player2_coord.y += 10.0
        }
        //Ball code.
        self.ball_coord += Vector2::new(-2.0, 0.0);
        if (self.ball_coord.x == self.player1_coord.x
            || self.ball_coord.x == self.player1_coord.x + 10.0)
            && self.ball_coord.y >= self.player1_coord.y
            && self.ball_coord.y <= self.player1_coord.y + 150.0
        {
            self.ball_update_position(1);
        }
        if (self.ball_coord.x == self.player2_coord.x
            || self.ball_coord.x == self.player2_coord.x + 10.0)
            && self.ball_coord.y >= self.player2_coord.y
            && self.ball_coord.y <= self.player2_coord.y + 150.0
        {
            self.ball_update_position(2);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        let rectangle_for_player_1 =
            graphics::Rect::new(self.player1_coord.x, self.player1_coord.y, 10.0, 150.0);
        let rectangle_for_player_2 =
            graphics::Rect::new(self.player2_coord.x, self.player2_coord.y, 10.0, 150.0);
        let ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.ball_coord,
            10.0,
            2.0,
            graphics::WHITE,
        )?;
        let r1 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rectangle_for_player_1,
            graphics::WHITE,
        )?;

        let r2 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rectangle_for_player_2,
            graphics::WHITE,
        )?;
        //Draw graphics to screen.
        graphics::draw(ctx, &r1, DrawParam::default())?;
        graphics::draw(ctx, &r2, DrawParam::default())?;
        graphics::draw(ctx, &ball, DrawParam::default())?;
        graphics::present(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    //Main context and event loop creation.
    let cb = ggez::ContextBuilder::new("Rust Pong!", "Brandon");
    let (ctx, events_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx).unwrap();
    //Makes the window the right size.
    graphics::set_drawable_size(ctx, 1280.0, 720.0).unwrap();
    graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, 1280.0, 720.0)).unwrap();
    graphics::set_window_title(ctx, "Rust Pong!");
    //Let us start the game!
    event::run(ctx, events_loop, state)
}
