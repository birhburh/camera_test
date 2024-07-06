use macroquad::prelude::*;

#[macroquad::main("Camera Test")]
async fn main() {
    set_pc_assets_folder("assets");
    macroquad_profiler::profiler(Default::default());

    let texture = load_texture("texture.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);

    loop {
        clear_background(BLACK);

        draw_texture(&texture, 0., 0., WHITE);

        next_frame().await;
    }
}
