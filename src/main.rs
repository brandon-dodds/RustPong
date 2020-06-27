#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cgmath;
use cgmath::{Point2, Vector2};
use ggez::graphics::DrawParam;
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::*;

const TARGET_FPS: u32 = 60;

enum CollideableObjects {
    PLAYER1,
    PLAYER2,
    TOP,
    BOTTOM,
}

struct Ball {
    coord: Point2<f32>,
    movement: Vector2<f32>,
    speed: f32,
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
            coord: Point2::new(640.0, 360.0),
            movement: Vector2::new(-5.0, 0.0),
            speed: 3.0,
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
    fn update_angle(&mut self, player_val: CollideableObjects) -> f32 {
        let base_angle: f32 = 75.0;
        match player_val {
            CollideableObjects::PLAYER1 | CollideableObjects::PLAYER2 => {
                let current_y_coord = match player_val {
                    CollideableObjects::PLAYER1 => self.player1_coord.y,
                    _ => self.player2_coord.y,
                };
                let paddle_middle = (current_y_coord * 2.0 + 150.0) / 2.0;
                let diff = paddle_middle - self.used_ball.coord.y;
                let normalized_angle: f32 = diff / ((current_y_coord + 150.0) / 2.0);
                let bounce_angle = normalized_angle * base_angle.to_radians();
                return bounce_angle;
            }
            _ => base_angle,
        }
    }
    fn ball_update_position(&mut self, player_val: CollideableObjects) {
        if self.used_ball.speed < 10.0 {
            self.used_ball.speed += 1.0;
        }
        let speed = self.used_ball.speed;
        match player_val {
            CollideableObjects::PLAYER1 => {
                self.used_ball.movement.x =
                    speed * self.update_angle(CollideableObjects::PLAYER1).cos();
                self.used_ball.movement.y =
                    speed * -self.update_angle(CollideableObjects::PLAYER1).sin();
            }
            CollideableObjects::PLAYER2 => {
                self.used_ball.movement.x =
                    speed * -self.update_angle(CollideableObjects::PLAYER2).cos();
                self.used_ball.movement.y =
                    speed * -self.update_angle(CollideableObjects::PLAYER2).sin();
            }
            CollideableObjects::TOP => {
                self.used_ball.movement.y =
                    speed * -self.update_angle(CollideableObjects::TOP).sin();
            }
            CollideableObjects::BOTTOM => {
                self.used_ball.movement.y =
                    speed * self.update_angle(CollideableObjects::BOTTOM).sin();
            }
        }
    }
    fn player_ball_collision(&mut self, player_val: CollideableObjects) {
        let x_coord = match player_val {
            CollideableObjects::PLAYER1 => self.player1_coord.x,
            _ => self.player2_coord.x,
        };
        let y_coord = match player_val {
            CollideableObjects::PLAYER1 => self.player1_coord.y,
            _ => self.player2_coord.y,
        };
        if self.used_ball.coord.x >= x_coord
            && self.used_ball.coord.x <= x_coord + 10.0
            && self.used_ball.coord.y >= y_coord
            && self.used_ball.coord.y <= y_coord + 150.0
        {
            self.ball_update_position(player_val);
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, TARGET_FPS) {
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
            self.used_ball.coord += self.used_ball.movement;
            if self.used_ball.movement.x < 0.0 {
                self.player_ball_collision(CollideableObjects::PLAYER1);
            } else {
                self.player_ball_collision(CollideableObjects::PLAYER2);
            }
            if self.used_ball.coord.y <= 0.0 {
                self.ball_update_position(CollideableObjects::TOP);
            }
            if self.used_ball.coord.y >= graphics::drawable_size(ctx).1 {
                self.ball_update_position(CollideableObjects::BOTTOM);
            }
            if self.used_ball.coord.x <= 0.0 {
                ggez::event::quit(ctx);
            }
            if self.used_ball.coord.x >= graphics::drawable_size(ctx).0 {
                ggez::event::quit(ctx);
            }
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
            self.used_ball.coord,
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
