use notan::draw::*;
use notan::prelude::*;
use pong_core::*;
use math::round;



const fn window_size() -> (usize, usize) {
    return (128, 64);
}

#[derive(AppState)]
struct State {
    game: PongGame,
}


fn setup(_gfx: &mut Graphics) -> State {
    State {
        game: PongGame::new(),
    }
}



#[notan_main]
fn main() -> Result<(), String> {

    const WIDTH: i32 = window_size().0 as i32;
    const HEIGHT: i32 = window_size().1 as i32;

    let window_config = WindowConfig::new()
        .title("pong-frontend")
        .size(WIDTH, HEIGHT)
        .vsync(true)
        .resizable(true)
        .min_size(WIDTH, HEIGHT);

    notan::init_with(setup).add_config(DrawConfig).add_config(window_config).draw(draw).build()
}


fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    
        if app.keyboard.is_down(KeyCode::Down) {
            state.game.left_y += 1;
            if state.game.left_y >= 45 {
                state.game.left_y = 45;
            }
        }
    

        if app.keyboard.is_down(KeyCode::Up) {
            if state.game.left_y <= 0 {
                state.game.left_y = 0;
            } else {
                state.game.left_y -= 1;
            }
        }
    
    

    draw.rect((state.game.left_x as f32, state.game.left_y as f32), (state.game.paddle_size_x, state.game.paddle_size_y));

    draw.rect((state.game.right_x as f32 - 6f32, state.game.right_y as f32), (state.game.paddle_size_x, state.game.paddle_size_y));

    draw.rect((state.game.ball_x as f32, state.game.ball_y as f32), (3f32, 3f32));

    state.game.time_step();

    gfx.render(&draw);
}
