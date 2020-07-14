pub struct Ball {
    // The current coordinate of the ball.
    pub coord: cgmath::Point2<f32>,
    // This is the movement vector that the ball moves along.
    pub movement: cgmath::Vector2<f32>,
    // The speed affects the movement vector in the ball_update_position() function.
    pub speed: f32,
}
