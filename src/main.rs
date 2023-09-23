use macroquad::prelude::*;

const SCREEN_HEIGHT: i32 = 600;
const SCREEN_WIDTH: i32 = 800;

const BLOCK_HEIGHT: i32 = SCREEN_HEIGHT / 20;
const BLOCK_WIDTH: i32 = SCREEN_WIDTH / 20;

const BALL_RADIUS: i32 = SCREEN_HEIGHT / 30;

const RACQUET_HEIGHT: i32 = SCREEN_HEIGHT / 60;
const RACQUET_WIDTH: i32 = SCREEN_WIDTH / 10;
const RACQUET_SPEED: f32 = 100.0;

enum HitBy {
    X,
    Y,
    XY,
}

struct Block {
    pos: Vec2,
    visible: bool,
}

impl Block {
    fn new(x: f32, y: f32) -> Block {
        Block {
            pos: Vec2 { x, y },
            visible: true,
        }
    }
    fn draw(&self) {
        if self.visible {
            draw_rectangle(
                self.pos.x + 2.0,
                self.pos.y + 2.0,
                (BLOCK_WIDTH - 4) as f32,
                (BLOCK_HEIGHT - 4) as f32,
                GREEN,
            )
        }
    }
    fn check_hit(&mut self, ball: &Ball) -> Option<HitBy> {
        if !self.visible {
            return None;
        }
        let block_rect = Rect {
            x: self.pos.x,
            y: self.pos.y,
            w: BLOCK_WIDTH as f32,
            h: BLOCK_HEIGHT as f32,
        };
        let ball_circ = Circle {
            x: ball.pos.x,
            y: ball.pos.y,
            r: BALL_RADIUS as f32,
        };

        if ball_circ.overlaps_rect(&block_rect) {
            self.visible = false;
            let ball_center = ball_circ.point();
            return if (block_rect.left()..block_rect.right()).contains(&ball_center.x) {
                Some(HitBy::X)
            } else if (block_rect.top()..block_rect.bottom()).contains(&ball_center.y) {
                Some(HitBy::Y)
            } else {
                Some(HitBy::XY)
            };
        }
        None
    }
}

struct Ball {
    pos: Vec2,
    direction_speed: Vec2,
    stuck: bool,
}

impl Ball {
    fn new() -> Ball {
        Ball {
            pos: Vec2 { x: 0.0, y: 0.0 },
            direction_speed: Vec2 { x: 0.0, y: 0.0 },
            stuck: true,
        }
    }
    fn bounce_x(&mut self) {
        self.direction_speed.x *= -1.0
    }
    fn bounce_y(&mut self) {
        self.direction_speed.y *= -1.0
    }
    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, BALL_RADIUS as f32, BLUE)
    }
    fn stick_to(&mut self, racquet: &Racquet) {
        self.pos.x = racquet.pos.x + (RACQUET_WIDTH / 2) as f32;
        self.pos.y = racquet.pos.y - BALL_RADIUS as f32 - 1.0;
    }
    fn update(&mut self) {
        if self.pos.x > (SCREEN_WIDTH - BALL_RADIUS) as f32 || self.pos.x < BALL_RADIUS as f32 {
            self.direction_speed.x *= -1.0
        }
        if self.pos.y < BALL_RADIUS as f32 {
            self.direction_speed.y *= -1.0
        }
        if self.pos.y > SCREEN_HEIGHT as f32 {
            self.direction_speed.y = 0.0;
            std::process::exit(0)
        }
        self.pos.x += self.direction_speed.x * get_frame_time();
        self.pos.y -= self.direction_speed.y * get_frame_time();
    }
}

struct Racquet {
    pos: Vec2,
    direction: i32,
}

impl Racquet {
    fn new() -> Racquet {
        Racquet {
            pos: Vec2 {
                x: (SCREEN_WIDTH / 2 - RACQUET_WIDTH / 2) as f32,
                y: (SCREEN_HEIGHT - RACQUET_HEIGHT) as f32,
            },
            direction: 0,
        }
    }
    fn draw(&self) {
        draw_rectangle(
            self.pos.x,
            self.pos.y,
            RACQUET_WIDTH as f32,
            RACQUET_HEIGHT as f32,
            RED,
        )
    }
    fn move_right(&mut self) {
        self.pos.x = (self.pos.x + RACQUET_SPEED * get_frame_time())
            .min((SCREEN_WIDTH - RACQUET_WIDTH) as f32);
        self.direction = 1;
    }
    fn move_left(&mut self) {
        self.pos.x = (self.pos.x - RACQUET_SPEED * get_frame_time()).max(0.0);
        self.direction = -1
    }
    fn stop(&mut self) {
        self.direction = 0;
    }
    fn check_hit(&self, ball: &Ball) -> Option<HitBy> {
        let racquet_rect = Rect {
            x: self.pos.x,
            y: self.pos.y,
            w: RACQUET_WIDTH as f32,
            h: RACQUET_HEIGHT as f32,
        };
        let ball_circ = Circle {
            x: ball.pos.x,
            y: ball.pos.y,
            r: BALL_RADIUS as f32,
        };

        if ball_circ.overlaps_rect(&racquet_rect) {
            let ball_center = ball_circ.point();
            if (racquet_rect.left()..racquet_rect.right()).contains(&ball_center.x) {
                return Some(HitBy::X);
            // } else if (racquet_rect.top()..racquet_rect.bottom()).contains(&ball_center.y) {
            //     return Some(HitBy::Y);
            } else {
                return Some(HitBy::XY);
            }
        }
        None
    }
}

#[macroquad::main("macronoid")]
async fn main() {
    let mut racquet = Racquet::new();
    let mut ball = Ball::new();
    ball.stick_to(&racquet);
    let mut blocks: Vec<Block> = Vec::new();
    for x in 0..20 {
        for y in 0..5 {
            blocks.push(Block::new(
                x as f32 * BLOCK_WIDTH as f32,
                y as f32 * BLOCK_HEIGHT as f32,
            ))
        }
    }
    loop {
        clear_background(WHITE);

        for block in blocks.iter_mut() {
            match block.check_hit(&ball) {
                Some(HitBy::X) => ball.bounce_y(),
                Some(HitBy::Y) => ball.bounce_x(),
                Some(HitBy::XY) => {
                    ball.bounce_x();
                    ball.bounce_y()
                }
                _ => {}
            }
            block.draw();
        }

        if is_key_down(KeyCode::Right) {
            racquet.move_right()
        } else if is_key_down(KeyCode::Left) {
            racquet.move_left()
        } else {
            racquet.stop()
        }
        if is_key_down(KeyCode::Space) && ball.stuck {
            ball.direction_speed.y = RACQUET_SPEED;
            ball.direction_speed.x = racquet.direction as f32 * RACQUET_SPEED;
            ball.stuck = false
        }
        match racquet.check_hit(&ball) {
            Some(HitBy::X) => {
                ball.bounce_y();
                ball.direction_speed.x += (racquet.direction as f32) * RACQUET_SPEED
            }
            // Some(HitBy::Y) => {
            //     ball.bounce_x();
            //     ball.direction_speed.x += (racquet.direction as f32) * RACQUET_SPEED
            // }
            Some(HitBy::XY) => {
                ball.bounce_x();
                ball.bounce_y();
                ball.direction_speed.x += (racquet.direction as f32) * RACQUET_SPEED
            }
            _ => {}
        }
        if ball.stuck {
            ball.stick_to(&racquet)
        }
        ball.update();
        ball.draw();
        racquet.draw();

        next_frame().await
    }
}
