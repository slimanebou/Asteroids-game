use chrono::Local;
use macroquad::prelude::*;

/// Draw all the elements that appear on screen at different stages.
pub fn menu_draw(screen_width: f32, screen_height: f32, game_over: bool, game_won: bool) {
    clear_background(BLACK);
    draw_text(
        "ASTEROIDS",
        screen_width / 2.0 - measure_text("ASTEROIDS", None, 40, screen_dpi_scale()).width / 2.0,
        screen_height / 2.0 - 50.0,
        40.0,
        WHITE,
    );
    draw_text(
        "Press ENTER to start",
        screen_width / 2.0 - measure_text("Press ENTER to start", None, 30, screen_dpi_scale()).width / 2.0,
        screen_height / 2.0,
        30.0,
        GRAY,
    );
    draw_text(
        "Press Esc to quit",
        screen_width / 2.0 - measure_text("Press Esc to quit", None, 30, screen_dpi_scale()).width / 2.0,
        screen_height / 2.0 + 50.0,
        30.0,
        GRAY,
    );

    if game_over {
        draw_text(
            "GAME OVER",
            screen_width / 2.0 - measure_text("GAME OVER", None, 60, screen_dpi_scale()).width / 2.0,
            screen_height / 2.0 - 150.0,
            60.0,
            RED,
        );
    } else if game_won {
        draw_text(
            "YOU WIN",
            screen_width / 2.0 - measure_text("YOU WIN", None, 60, screen_dpi_scale()).width / 2.0,
            screen_height / 2.0 - 150.0,
            60.0,
            GREEN,
        );
    }
}

/// Draw the debug interface and information about the game state
pub fn draw_simulation(
    debug: bool,
    cycle: u128,
    number_of_asteroids: usize,
    fps: u32,
    score: u128,
    simulation_speed: f64,
) {
    if debug {
        draw_text(
            &(format!("Cycle:{}", cycle)).to_string(),
            10.0,
            50.0,
            48.0,
            RED,
        );
        draw_text(
            &(format!("FPS:{}", fps)).to_string(),
            10.0,
            150.0,
            48.0,
            GREEN,
        );
        draw_text(
            &(format!("Time:{}", Local::now().format("%H:%M:%S"))),
            10.0,
            200.0,
            48.0,
            YELLOW,
        );
        draw_text(
            &(format!("Speed factor:{}x", simulation_speed)),
            (screen_width()
                - measure_text(
                    &format!("Speed factor:{}x", simulation_speed),
                    None,
                    36,
                    screen_dpi_scale(),
                )
                .width)
                / 2.0,
            25.0,
            36.0,
            GOLD,
        );
    }
    // If not in debug mode
    else {
        draw_text(
            &(format!("FPS:{}", fps)).to_string(),
            10.0,
            50.0,
            48.0,
            GREEN,
        );
    }
    // Always draw
    draw_text(
        &(format!("Astéroides:{}", number_of_asteroids)).to_string(),
        10.0,
        100.0,
        48.0,
        BLUE,
    );
    draw_text(
        &(format!("Score:{}", score)).to_string(),
        screen_width()
            - measure_text(&format!("Score:{}", score).to_string(), None, 48, 1.0).width
            - 10.0,
        50.0,
        48.0,
        WHITE,
    );
}
