use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Camera Test".to_owned(),
        platform: miniquad::conf::Platform {
            blocking_event_loop: true,
            ..Default::default()
        },
        // high_dpi: true,
        window_height: 200,
        window_width: 200,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");

    let texture = load_texture("texture.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);

    loop {
        clear_background(BLACK);

        set_default_camera();

        // set_camera(&Camera2D::from_display_rect(Rect::new(
        //     0.0, 0.0, 16.0, 16.0,
        // )));

        let window_size = vec2(screen_width(), screen_height());
        dbg!(window_size);
        dbg!(screen_dpi_scale());
        dbg!(macroquad::miniquad::window::screen_size());
        set_camera(
            &Camera2D::from_display_rect(Rect::new(
                0.0,
                window_size.y,
                window_size.x,
                -window_size.y,
            ))
        );
        draw_texture(&texture, 0., 0., WHITE);
        draw_line(0., 1., 16., 1., 2., RED);

        next_frame().await;
    }
}
