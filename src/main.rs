#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cgmath;
use cgmath::{Point2, Vector2};
use ggez::graphics::DrawParam;
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::*;

//The ball struct.

struct Ball {
    ball_coord: Point2<f32>,
    ball_movement: Vector2<f32>,
    ball_speed: f32,
}

//The main state of the program (players, balls).
struct MainState {
    player1_coord: Vector2<f32>,
    player2_coord: Vector2<f32>,
    used_ball: Ball,
}
//Implements the main state. New is created on creation of the game state.
impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let player1_coord = Vector2::new(100.0, 285.0);
        let player2_coord = Vector2::new(1170.0, 285.0);
        let used_ball = Ball {
            ball_coord: Point2::new(640.0, 360.0),
            ball_movement: Vector2::new(-5.0, 0.0),
            ball_speed: 3.0,
        };
        let s = MainState {
            player1_coord,
            player2_coord,
            used_ball,
        };

        Ok(s)
    }
    //Resetting the ball position/Handling the physics of ball.
    //Maths works as follows, find the difference between the ball y coord and the middle of the bat.
    // Use trigonometry to send the ball at an angle according to this difference.
    fn ball_update_position(&mut self, player_val: i8) {
        let degree: f32 = 75.0;
        if self.used_ball.ball_speed < 12.0 {
            self.used_ball.ball_speed += 1.0;
        }
        let speed = self.used_ball.ball_speed;
        println!("{}", speed);
        if player_val == 1 {
            let paddle_middle = (self.player1_coord.y * 2.0 + 150.0) / 2.0;
            let diff = paddle_middle - self.used_ball.ball_coord.y;
            let normalized_value: f32 = diff / ((self.player1_coord.y + 150.0) / 2.0);
            let bounce_angle = normalized_value * degree.to_radians();
            self.used_ball.ball_movement.x = speed * bounce_angle.cos();
            self.used_ball.ball_movement.y = speed * -bounce_angle.sin();
        } else if player_val == 2 {
            let paddle_middle = (self.player2_coord.y * 2.0 + 150.0) / 2.0;
            let diff = paddle_middle - self.used_ball.ball_coord.y;
            let normalized_value: f32 = diff / ((self.player2_coord.y + 150.0) / 2.0);
            let bounce_angle: f32 = normalized_value * degree.to_radians();
            self.used_ball.ball_movement.x = speed * -bounce_angle.cos();
            self.used_ball.ball_movement.y = speed * -bounce_angle.sin();
        } else if player_val == 3 {
            self.used_ball.ball_movement.y = speed * -degree.sin();
        } else if player_val == 4 {
            self.used_ball.ball_movement.y = speed * degree.sin();
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
        self.used_ball.ball_coord += self.used_ball.ball_movement;
        if self.used_ball.ball_coord.x >= self.player1_coord.x
            && self.used_ball.ball_coord.x <= self.player1_coord.x + 10.0
            && self.used_ball.ball_coord.y >= self.player1_coord.y
            && self.used_ball.ball_coord.y <= self.player1_coord.y + 150.0
        {
            self.ball_update_position(1);
        }
        if self.used_ball.ball_coord.x >= self.player2_coord.x
            && self.used_ball.ball_coord.x <= self.player2_coord.x + 10.0
            && self.used_ball.ball_coord.y >= self.player2_coord.y
            && self.used_ball.ball_coord.y <= self.player2_coord.y + 150.0
        {
            self.ball_update_position(2);
        }
        if self.used_ball.ball_coord.y <= 0.0 {
            self.ball_update_position(3);
        }
        if self.used_ball.ball_coord.y >= graphics::drawable_size(ctx).1 {
            self.ball_update_position(4);
        }
        if self.used_ball.ball_coord.x <= 0.0 {
            ggez::event::quit(ctx);
        }
        if self.used_ball.ball_coord.x >= graphics::drawable_size(ctx).0 {
            ggez::event::quit(ctx);
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
            self.used_ball.ball_coord,
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
