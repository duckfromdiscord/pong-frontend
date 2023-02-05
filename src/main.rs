use notan::draw::*;
use notan::prelude::*;
use pong_core::*;


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

    let window_config = WindowConfig::new()
        .title("pong-frontend")
        .size(128, 64)
        .vsync(true)
        .resizable(true)
        .min_size(128, 64);

    notan::init_with(setup).add_config(DrawConfig).add_config(window_config).draw(draw).build()
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);


    draw.rect((state.game.left_x as f32, state.game.left_y as f32), (6f32, 20f32));

    draw.rect((state.game.right_x as f32 - 6f32, state.game.right_y as f32), (6f32, 20f32));

    draw.rect((state.game.ball_x as f32, state.game.ball_y as f32), (3f32, 3f32));

    state.game.time_step();

    gfx.render(&draw);
}
