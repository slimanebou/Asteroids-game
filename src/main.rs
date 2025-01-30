use general::Gamestate;
use macroquad::prelude::*;
use std::collections::HashMap;

// Asteroid
mod asteroid;
use asteroid::{Asteroid, TEXTURES};

// Vaisseau
use spaceship::Spaceship;
mod spaceship;

//missile
mod missile;
use missile::Missile;

// Menu
mod menus;

// Game
mod general;

// The precision of the simulation / the duration of one tick
const TICKS: f64 = 1.0 / 60.0;

/// Processes user input and applies changes to the game state.
///
/// This function handles different operations, such as moving objects,
/// firing missiles, or pausing the game, based on the player's input.
/// It updates the provided [`Gamestate`] structure accordingly.
///
/// # Parameters
/// - `gamestate`: A mutable reference to the [`Gamestate`]. This is required
///   to update game elements like the spaceship, asteroids, or game status.
///
/// # Returns
/// - `true` to exit the game.
/// - `false` to continue the game.
fn handle_input(
    gamestate: &mut Gamestate
) -> bool {
    if is_key_pressed(KeyCode::F3) {
        gamestate.debug = !gamestate.debug;
    }

    // Gérer les entrées clavier pour contrôler le vaisseau
    if is_key_down(KeyCode::Up) && gamestate.simulation_speed > 0.0 {
        gamestate.spaceship.move_spaceship(gamestate.delta_time, true);
    }
    if is_key_down(KeyCode::Down) && gamestate.simulation_speed > 0.0 {
        gamestate.spaceship.move_spaceship(gamestate.delta_time, false);
    }
    if is_key_down(KeyCode::Left) && gamestate.simulation_speed > 0.0 {
        gamestate.spaceship.add_rotation(-gamestate.spaceship.turn_rate * gamestate.delta_time as f32);
    }
    if is_key_down(KeyCode::Right) && gamestate.simulation_speed > 0.0 {
        gamestate.spaceship.add_rotation(gamestate.spaceship.turn_rate * gamestate.delta_time as f32);
    }

    if is_key_pressed(KeyCode::Space) | (is_mouse_button_down(MouseButton::Left) && gamestate.debug)
        && gamestate.simulation_speed > 0.0
    {
        // Use spaceship's rotation directly for the missile's direction
        gamestate.missiles.push(Missile::new(
            gamestate.spaceship.position,
            gamestate.spaceship.max_speed,
            gamestate.spaceship.rotation,
        ))
    }

    if is_key_pressed(KeyCode::S) && gamestate.simulation_speed > 0.0 {
        gamestate.spaceship.speed = 0.0;
    }

    if is_key_down(KeyCode::LeftControl) {
        gamestate.simulation_speed = 0.0;
    } else if is_key_down(KeyCode::LeftShift) {
        gamestate.simulation_speed = 0.1;
    } else if is_key_down(KeyCode::Tab) {
        gamestate.simulation_speed = 10.0;
    } else {
        gamestate.simulation_speed = if gamestate.game_started { 1.0 } else { 0.0 };
    }

    if !gamestate.game_started {
        if is_key_pressed(KeyCode::Escape) {
            return true;
        }

        if is_key_pressed(KeyCode::Enter) {
            gamestate.game_started = true;
            gamestate.lives = 3; // Réinitialise les vies
            gamestate.score = 0; // Réinitialise le score
            gamestate.game_over = false;
            gamestate.game_won = false;
            gamestate.asteroids.clear();
            gamestate.missiles.clear();
            gamestate.spaceship = Spaceship::new();
            for _ in 1..=20 {
                gamestate.asteroids.push(Asteroid::default());
            }
        }
    }

    if is_key_pressed(KeyCode::Escape) {
        gamestate.game_started = false;
    }

    false
}

/// This function remove elements specifically for asteroids based on a
/// collection of indices. It does so by first sorting it so that indices
/// are removed last to first to avoid issues. There must not be duplicates !
fn remove_asteroid(vector: &mut Vec<Asteroid>, indices: &mut Vec<usize>) {
    // To prevent a crash when the Vec is modified unexpectedly
    indices.sort_by(|a, b| b.cmp(a));
    for &mut index in indices {
        vector.remove(index);
    }
}

/// Will detect collisions between the spaceship and asteroids and
/// execute necessary operations to match the expected behaviour.
fn check_collision_spaceship_asteroid(
    spaceship: &Spaceship,
    asteroids: &mut Vec<Asteroid>,
) -> bool {
    let mut to_add: Vec<Asteroid> = Vec::new();
    let mut to_remove_asteroids: Vec<usize> = Vec::new();
    let mut result: bool = false;

    for asteroid in asteroids.iter_mut().enumerate() {
        // Calcul de la distance entre le vaisseau et l'astéroïde
        let distance = (spaceship.position - asteroid.1.position).length();

        let collision_radius =
            asteroid.1.size as f32 * asteroid.1.scale / 2.0 + spaceship.get_collision_radius();

        // Si la distance est inférieure au rayon de collision, il y a collision
        if distance < collision_radius {
            to_add = asteroid.1.split(true);
            to_remove_asteroids.push(asteroid.0);
            result = true;
        }
    }
    remove_asteroid(asteroids, &mut to_remove_asteroids);
    asteroids.extend(to_add);
    result
}

/// Will detect collisions between missiles and asteroids and
/// execute necessary operations to match the expected behaviour.
fn check_collision_asteroid_missile(
    gamestate: &mut Gamestate
) {
    let mut to_remove_asteroids = Vec::new();
    let mut to_remove_missiles = Vec::new();
    let size_to_score = Vec::from([3, 2, 1]);
    let mut to_add: Vec<Asteroid> = Vec::new();

    for (asteroid_index, asteroid) in gamestate.asteroids.iter_mut().enumerate() {
        for (missile_index, missile) in gamestate.missiles.iter_mut().enumerate() {
            let distance = (asteroid.position - missile.position).length();
            let collision_radius = asteroid.size as f32 * asteroid.scale / 2.0 + missile.size;

            if missile.active && distance < collision_radius {
                to_add.extend(asteroid.split(gamestate.number_of_asteroids < gamestate.asteroid_limit));
                // Prevent crash by not duplicating the index
                if !to_remove_asteroids.contains(&asteroid_index) {
                    // Marque l'astéroïde pour suppression
                    to_remove_asteroids.push(asteroid_index);
                }
                if !to_remove_missiles.contains(&missile_index) {
                    // Marque le missile pour suppression
                    to_remove_missiles.push(missile_index);
                }
                // The score gained depends on the properties of the asteroid (TO DO)
                gamestate.score += 100 * size_to_score[asteroid.size as usize - 1];
                // On ne vérifie plus ce missile pour cet astéroïde
                break;
            }
        }
    }

    // Ajuste le vecteur en enlevant les missiles et les astéroides détruits et en rajoutant les petits
    remove_asteroid(&mut gamestate.asteroids, &mut to_remove_asteroids);
    to_remove_missiles.sort_by(|a, b| b.cmp(a));
    for idx in to_remove_missiles {
        gamestate.missiles.remove(idx);
    }
    gamestate.asteroids.extend(to_add);
    gamestate.number_of_asteroids = gamestate.asteroids.len();
}

/*
For reference visit https://macroquad.rs/examples/
Altough it's outdated and vastly different
*/

/// The main entry point of the Asteroids game.
///
/// This function initializes the game environment, including window parameters, textures, and game state.
/// It then enters the game loop, which:
/// - Handles player input.
/// - Updates the game state (e.g., physics, collisions, and game logic).
/// - Renders the game visuals, including the background, asteroids, spaceship, and missiles.
///
/// The game loop is designed to be frame-rate independent using a delta-time (DT) system. 
/// This ensures smooth animations and consistent behavior, regardless of the frame rate.
///
/// # Features
/// - **Initialization**: Sets up the game window and loads necessary textures.
/// - **Game Loop**: 
///   - Handles input from the player (e.g., spaceship movement, firing missiles).
///   - Updates the positions, rotations, and states of game objects.
///   - Checks for and resolves collisions (e.g., between asteroids, missiles, and the spaceship).
///   - Displays the current game state, including debug information if enabled.
/// - **Game State Management**:
///   - Ends the game when the player loses all lives or destroys all asteroids.
///   - Manages transitions to the main menu or victory screen.
///
/// # Delta-Time System
/// The delta-time (DT) system ensures physics calculations and movements are time-based rather than 
/// frame-based. This provides consistent gameplay across varying system performance.
///
/// # Window Initialization
/// - The game starts in fullscreen mode.
/// - Waits until the screen dimensions are updated from the default `800x600`.
///
/// Once launched, use the appropriate input controls to play.
///
/// # Notes
/// - This function runs asynchronously due to the use of the `macroquad` library.
/// - Ensure all textures are placed in the correct directory to avoid runtime errors.
///
/// # See Also
/// - [`Gamestate`](./gamestate.rs): The core structure that tracks the game's state.
#[macroquad::main("Asteroids")]
async fn main() {
    // Initialisation
    macroquad::window::set_fullscreen(true);
    while screen_width() == 800.0 || screen_height() == 600.0 {
        next_frame().await
    }

    let mut gamestate = Gamestate::new();

    let mut textures = HashMap::new();
    for texture in TEXTURES.iter() {
        textures.insert(
            texture,
            load_texture(texture).await.expect("Failed to load texture"),
        );
        println!("[INFO]: Loaded texture: {texture}")
    }

    let background_texture = load_texture("./assets/textures/background.png")
        .await
        .expect("Failed to load background texture");
    background_texture.set_filter(FilterMode::Nearest);

    let mut previous_time = 0.0;
    let mut accumulator = 0.0;
    let mut fps_cooldown = get_time();
    let mut fps = macroquad::time::get_fps() as u32;

    loop {
        gamestate.delta_time = (get_time() - previous_time) * gamestate.simulation_speed;
        previous_time = get_time();

        if handle_input(&mut gamestate) {
            println!("Exiting the game...");
            break;
        }

        gamestate.missiles.retain(|m| m.active);

        if get_time() - fps_cooldown >= 0.25 {
            fps = macroquad::time::get_fps() as u32;
            fps_cooldown = get_time()
        }

        let mut asteroid_index_to_remove: Vec<usize> = Vec::new();

        // Jeu principal
        if check_collision_spaceship_asteroid(&gamestate.spaceship, &mut gamestate.asteroids) {
            gamestate.lives = gamestate.lives.saturating_sub(1);
            // Réinitialise la position du vaisseau
            gamestate.spaceship = Spaceship::new();
        }

        // Si le joueur perd ses 3 vies, revenir au menu principal
        if gamestate.lives == 0 {
            gamestate.game_started = false;
            gamestate.game_over = true;
            // Retour au menu principal
        }

        accumulator += gamestate.delta_time;
        while accumulator >= gamestate.delta_time {
            gamestate.loop_number += 1;

            for asteroid in &mut gamestate.asteroids.iter_mut().enumerate() {
                if asteroid.1.size == 0 && !asteroid_index_to_remove.contains(&asteroid.0) {
                    asteroid_index_to_remove.push(asteroid.0);
                }
            }

            let mut to_add: Vec<Asteroid> = Vec::new();
            for index in &asteroid_index_to_remove {
                if gamestate.number_of_asteroids <= gamestate.asteroid_limit {
                    to_add.extend(gamestate.asteroids[*index].split(gamestate.number_of_asteroids < gamestate.asteroid_limit))
                };
            }

            gamestate.asteroids.extend(to_add);
            gamestate.number_of_asteroids = gamestate.asteroids.len();

           check_collision_asteroid_missile(&mut gamestate);
            accumulator -= TICKS;
        }

        for asteroid in &mut gamestate.asteroids {
            asteroid.add_rotation(asteroid.turn_rate * gamestate.delta_time as f32);
            asteroid.move_object(gamestate.delta_time);
        }
        gamestate.spaceship.update(gamestate.delta_time);
        for missile in &mut gamestate.missiles {
            missile.update(gamestate.delta_time);
        }

        remove_asteroid(&mut gamestate.asteroids, &mut asteroid_index_to_remove);

        // Affichages
        if !gamestate.debug {
        draw_texture_ex(
            &background_texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );}

        for asteroid in &mut gamestate.asteroids.iter_mut().enumerate() {
            asteroid.1.draw_self(
                textures.get(&asteroid.1.texture.to_string()).unwrap(),
                gamestate.debug,
            );
        }
        gamestate.spaceship.draw(25.0, gamestate.debug);
        for missile in &gamestate.missiles {
            missile.draw();
        }

        if !gamestate.game_started {
            menus::menu_draw(screen_width(), screen_height(), gamestate.game_over, gamestate.game_won);
        } else {
            menus::draw_simulation(
                gamestate.debug,
                gamestate.loop_number,
                gamestate.number_of_asteroids,
                fps,
                gamestate.score,
                gamestate.simulation_speed,
            );

            if gamestate.asteroids.is_empty() {
                gamestate.game_won = true;
                gamestate.game_started = false;
            }
        }

        next_frame().await;
    }
}
