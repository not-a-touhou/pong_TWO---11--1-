use notan::{
    prelude::*,
    draw::*,
};

use rand::prelude::*;

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;

const PADDLE_HEIGHT: f32 = 128.0;
const PADDLE_WIDTH: f32 = 32.0;
const PADDLE_DIST_FROM_EDGE: f32 = 64.0;
const PADDLE_SPEED: f32 = 300.0;

const BALL_SIZE: f32 = 32.0;

enum PaddleType {
    Player,
    Enemy,
}

struct Paddle {
    x_pos: f32,
    y_pos: f32,
    paddle_type: PaddleType,
}

struct Ball {
    x_pos: f32,
    y_pos: f32,
    x_vel: f32,
    y_vel: f32,
}

#[derive(AppState)]
struct Game {
    font: Font,
    last_key: Option<KeyCode>,
    elapsed: f32,
    rng: ThreadRng,
    player: Paddle,
    enemy: Paddle,
    ball: Ball,
}


fn collides_ball_paddle(paddle: &mut Paddle, ball: &mut Ball) -> bool {
    if  paddle.x_pos + PADDLE_WIDTH  > ball.x_pos             &&
        paddle.x_pos                 < ball.x_pos + BALL_SIZE &&
        paddle.y_pos + PADDLE_HEIGHT > ball.y_pos             &&
        paddle.y_pos                 < ball.y_pos + BALL_SIZE
    {
        return true
    } else {
        return false
    }
}


fn setup(gfx: &mut Graphics) -> Game {
    let font = gfx
        .create_font(include_bytes!("../assets/Ubuntu-Regular.ttf"))
        .unwrap();
    
    let rng = rand::thread_rng();

    Game {
        font, 
        last_key: None,
        player: Paddle {
            x_pos: PADDLE_DIST_FROM_EDGE,
            y_pos: (SCREEN_HEIGHT / 2.0) - (PADDLE_HEIGHT / 2.0),
            paddle_type: PaddleType::Player,
        },
        elapsed: 0.0,
        rng,
        enemy: Paddle { 
            x_pos: SCREEN_WIDTH - PADDLE_DIST_FROM_EDGE - PADDLE_WIDTH, 
            y_pos: (SCREEN_HEIGHT / 2.0) - (PADDLE_HEIGHT / 2.0), 
            paddle_type: PaddleType::Enemy,
        },
        ball: Ball { 
            x_pos: (SCREEN_WIDTH / 2.0) - (BALL_SIZE / 2.0), 
            y_pos: (SCREEN_HEIGHT / 2.0) - (BALL_SIZE / 2.0), 
            x_vel: -100.0, 
            y_vel: -1.0, 
        }
    }
}

fn update(app: &mut App, state: &mut Game) {
    state.elapsed = app.system_timer.delta().as_secs_f32();

    if app.keyboard.was_pressed(KeyCode::Escape) {
        println!("Exiting game TwT");
        app.exit();
    } 
    
    if app.keyboard.is_down(KeyCode::W) && state.player.y_pos > 0.0 {
        state.player.y_pos -= PADDLE_SPEED * state.elapsed;
    } else if app.keyboard.is_down(KeyCode::S) && state.player.y_pos + PADDLE_HEIGHT < SCREEN_HEIGHT {
        state.player.y_pos += PADDLE_SPEED * state.elapsed;
    }

    if app.keyboard.is_down(KeyCode::Up) && state.enemy.y_pos > 0.0 {
        state.enemy.y_pos -= PADDLE_SPEED * state.elapsed;
    } else if app.keyboard.is_down(KeyCode::Down) && state.enemy.y_pos + PADDLE_HEIGHT < SCREEN_HEIGHT {
        state.enemy.y_pos += PADDLE_SPEED * state.elapsed;
    }


    if collides_ball_paddle(&mut state.player, &mut state.ball) {
        state.ball.x_vel *= -1.0;
        let collision_dist: f32 = (state.ball.y_pos + (BALL_SIZE/2.0)) - (state.player.y_pos +(PADDLE_HEIGHT/2.0));
        state.ball.y_vel = collision_dist * state.rng.gen::<f32>();
    }
    if collides_ball_paddle(&mut state.enemy, &mut state.ball) {
        state.ball.x_vel *= -1.0;
        let collision_dist: f32 = (state.ball.y_pos + (BALL_SIZE/2.0)) - (state.enemy.y_pos +(PADDLE_HEIGHT/2.0));
        state.ball.y_vel = collision_dist * (state.rng.gen::<f32>() * 3.0);
    }
    if (state.ball.y_pos + BALL_SIZE > SCREEN_HEIGHT) || (state.ball.y_pos < 0.0) {
        state.ball.y_vel *= -1.0;
    }

    state.ball.x_pos = state.ball.x_pos + state.ball.x_vel * state.elapsed;
    state.ball.y_pos = state.ball.y_pos + state.ball.y_vel * state.elapsed;
}

fn draw(gfx: &mut Graphics, state: &mut Game) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw.rect((state.player.x_pos, state.player.y_pos), (PADDLE_WIDTH, PADDLE_HEIGHT))
        .color(Color::WHITE);

    draw.rect((state.enemy.x_pos, state.enemy.y_pos), (PADDLE_WIDTH, PADDLE_HEIGHT))
        .color(Color::WHITE);

    draw.rect((state.ball.x_pos, state.ball.y_pos), (BALL_SIZE, BALL_SIZE))
        .color(Color::WHITE);

    gfx.render(&draw);
}

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::default()
        .set_decorations(false)
        .set_size(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);

    notan::init_with(setup)
        .add_config(win)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}   