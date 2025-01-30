use ::rand::distributions::{Distribution, WeightedIndex};
use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use std::f32::consts::PI;

use ::std::fs;
use lazy_static::lazy_static;
use std::path::PathBuf;

/// Will get all the file names from the specified directory, not exclusive to images
pub fn get_textures(dir: &str) -> Vec<String> {
    let mut filenames: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        // Utilisation de flatten() pour éviter l'usage de if let
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(filename) = path.file_name() {
                    if let Some(filename_str) = filename.to_str() {
                        filenames.push(String::from(dir) + "/" + filename_str);
                    }
                }
            }
        }
    }
    filenames
}

impl Default for Asteroid {
    fn default() -> Self {
        let mut rng = thread_rng();
        let new_properties = Self::new_properties();
        Self {
            position: Self::new_alea_pos(),
            speed: new_properties.2,
            size: 3,
            scale: 40.0,
            rotation: Self::new_rotation(),
            direction: rng.gen_range(0.0..=2.0 * PI),
            speed_multiplier: new_properties.1,
            turn_rate: rng.gen_range(0.5..1.5) * if rng.gen() { 1.0 } else { -1.0 },
            texture: TEXTURES[Self::create_weights()].to_string(),
        }
    }
}

pub struct Asteroid {
    pub position: Vec2,
    pub speed: f32,
    pub size: u8,
    pub scale: f32,
    pub rotation: f32,
    pub direction: f32,
    pub speed_multiplier: f32,
    pub turn_rate: f32, // °/s
    pub texture: String,
}

impl Asteroid {
    pub const ASTEROID_INIT_SIZE: f32 = 60.0;

    /// The default builder which may accept set values.
    pub fn new(
        position: Option<Vec2>,
        speed: Option<f32>,
        size: Option<u8>,
        scale: Option<f32>,
        rotation: Option<f32>,
        direction: Option<f32>,
        speed_multiplier: Option<f32>,
        turn_rate: Option<f32>,
        texture: Option<String>,
    ) -> Self {
        let mut asteroid = Self::default();

        // Override fields only if a value is provided
        if let Some(position) = position {
            asteroid.position = position;
        }
        if let Some(speed) = speed {
            asteroid.speed = speed;
        }
        if let Some(size) = size {
            asteroid.size = size;
        }
        if let Some(scale) = scale {
            asteroid.scale = scale;
        }
        if let Some(rotation) = rotation {
            asteroid.rotation = rotation;
        }
        if let Some(direction) = direction {
            asteroid.direction = direction;
        }
        if let Some(speed_multiplier) = speed_multiplier {
            asteroid.speed_multiplier = speed_multiplier;
        }
        if let Some(turn_rate) = turn_rate {
            asteroid.turn_rate = turn_rate;
        }
        if let Some(texture) = texture {
            asteroid.texture = texture;
        }

        asteroid
    }

    /// Moves the object by adding rotation within the bounds of [-2PI;2PI]
    pub fn add_rotation(&mut self, amount: f32) {
        self.rotation = (self.rotation + amount) % (PI * 2.0);
    }

    /// Moves the object based on its speed, applying inertia.
    pub fn move_object(&mut self, delta_time: f64) {
        let direction = vec2(self.direction.cos(), self.direction.sin());
        self.position += direction * self.speed * self.speed_multiplier * delta_time as f32;
        // Move at the opposite edge
        self.position = Self::bound_pos(self.position);
    }

    /// Generates a random position near one of the screen edges.
    fn new_alea_pos() -> Vec2 {
        let mut rng = thread_rng();
        let nearpos: f32 = rng.gen_range(Self::ASTEROID_INIT_SIZE / 2.0..=Self::ASTEROID_INIT_SIZE);
        // 1 = top, 2 = right, 3 = bottom, 4 = left
        let nearside = rng.gen_range(1..=4);
        let xpos: f32 = match nearside {
            2 => screen_width() - nearpos,
            4 => nearpos,
            _ => rng.gen_range(0.0..=screen_width()),
        };
        let ypos: f32 = match nearside {
            1 => nearpos,
            3 => screen_height() - nearpos,
            _ => rng.gen_range(0.0..=screen_height()),
        };
        vec2(xpos, ypos)
    }

    /// Create properties based on each other and assign them to a tuple for the constructor
    fn new_properties() -> (u8, f32, f32) {
        let mut rng = thread_rng();
        let size = rng.gen_range(1..=3);
        let speed_multiplier = rng.gen_range(0.4..=1.5);
        let size_to_speed = match size {
            1 => 3.0,
            2 => 2.0,
            _ => 1.0,
        };

        (
            size,
            speed_multiplier,
            size_to_speed * speed_multiplier * 50.0,
        )
    }

    /// Generate a random rotation between \[0;2PI]
    fn new_rotation() -> f32 {
        let mut rng = thread_rng();
        rng.gen_range(0.0..=2.0 * PI)
    }

    /// Generate the rariry weights of the different kinds of asteroids
    /// The first asteroid variant appears 95% of the time while
    /// the rest is distributed equally in the remaining 5%
    fn create_weights() -> usize {
        let mut rng = thread_rng();
        // Define the weight for the first item (95%) and equal weights for the rest (5%)
        let first_rariry = 85.0;
        let equal_weight = (100.0 - first_rariry) / (TEXTURES.len() - 1) as f32;

        // Create the weights vector
        let mut weights = vec![first_rariry];
        weights.extend(vec![equal_weight; TEXTURES.len() - 1]);
        WeightedIndex::new(&weights).unwrap().sample(&mut rng)
    }

    fn bound_pos(mut pos: Vec2) -> Vec2 {
        pos.x = Self::bound_to(pos.x, screen_width());
        pos.y = Self::bound_to(pos.y, screen_height());
        pos
    }

    fn bound_to(coord: f32, max: f32) -> f32 {
        if coord < 0.0 {
            max
        } else if coord > max {
            0.0
        } else {
            coord
        }
    }

    /// Create two new asteroids from the attributes of the parent asteroid
    pub fn split(&mut self, can_add: bool) -> Vec<Asteroid> {
        let mut output = Vec::new();
        let mut rng = thread_rng();
        if self.size - 1 != 0 {
            output.push(Asteroid::new(
                Some(self.position),
                Some(self.speed * -rng.gen_range(1.0..=1.75)),
                Some(if self.size >= 1 { self.size - 1 } else { 0 }),
                None,
                Some(-self.rotation + PI / 4.0), // Instantly invert the rotation
                Some(-self.direction - PI / 4.0),
                Some(self.speed_multiplier),
                Some(-self.turn_rate * rng.gen_range(1.0..=2.0)), // Randomly change the turn rate while also inverting it
                Some(self.texture.clone()),
            ))
        };
        if can_add && self.size - 1 != 0 {
            output.push(Asteroid::new(
                Some(self.position),
                Some(self.speed * -rng.gen_range(1.0..=2.0)),
                Some(if self.size > 1 { self.size - 1 } else { 0 }),
                None,
                Some(-self.rotation - PI / 4.0),
                Some(-self.direction + PI / 4.0),
                Some(self.speed_multiplier),
                Some(self.turn_rate * rng.gen_range(1.0..=3.5)),
                Some(self.texture.clone()),
            ));
        }
        output
    }

    /// For the debug mode. Will draw lines to indicate the direction and the orientation
    pub fn draw_trajectory(&self) {
        // Define the arrow length and compute the direction where the asteroid is moving
        let arrow_length = 40.0;
        // Normalize to get direction

        // Get the asteroid's current position and rotation
        let start = self.position;

        // Calculate the direction of the arrow based on the asteroid's rotation
        let direction = vec2(self.direction.cos(), self.direction.sin()) * self.direction.signum();
        let rotation = vec2(self.rotation.cos(), self.rotation.sin());

        // Calculate the end point of the arrows
        let end_rotation = start + rotation * arrow_length;
        let end_direction = start + direction * arrow_length;

        // Draw the trajectory arrow
        draw_line(start.x, start.y, end_direction.x, end_direction.y, 2.0, RED);
        // Draw the rotation direction of the texture arrow
        draw_line(
            start.x,
            start.y,
            end_rotation.x,
            end_rotation.y,
            2.0,
            YELLOW,
        );
    }

    /// Display of the asteroid.
    pub fn draw_self(&self, texture: &Texture2D, debug: bool) {
        // Ensure the size_multiplier is cast to f32 to use with scale
        let adjusted_scale = self.scale * self.size as f32;
        let font_size = 20.0;
        let position = self.position;

        draw_texture_ex(
            texture,
            // Center the texture to the asteroid's center
            position.x - adjusted_scale / 2.0,
            position.y - adjusted_scale / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(adjusted_scale, adjusted_scale)),
                rotation: self.rotation,
                ..Default::default()
            },
        );

        if debug {
            // Attributes
            let size_to_speed = match self.size {
                1 => 3.5,
                2 => 2.0,
                _ => 1.0,
            };
            let mut texts = Vec::from([
                format!("x:{:.2} y:{:.2}", position.x, position.y),
                format!("Size:{}", self.size),
                format!(
                    "Speed:{:.2}px/s",
                    self.speed * self.speed_multiplier * size_to_speed
                ),
                format!(
                    "Rotation:{}|{:.3}rad",
                    if self.turn_rate.signum() == 1.0 {
                        "R"
                    } else {
                        "L"
                    },
                    self.rotation
                ),
                format!("Turn Rate:{:.3}rad/s", self.turn_rate),
                format!("Direction: {:.3}rad", self.direction),
                format!("Speed modifier:{:.2}%", (self.speed_multiplier * 80.0)),
                format!(
                    "Variant:{}",
                    PathBuf::from(self.texture.clone())
                        .file_stem()
                        .unwrap()
                        .to_string_lossy()
                ),
            ]);

            let mut debug_text_sizes: Vec<u16> = Vec::new();

            for field in &texts {
                debug_text_sizes.push(
                    measure_text(
                        &field.to_string(),
                        None,
                        font_size as u16,
                        screen_dpi_scale(),
                    )
                    .width as u16,
                );
            }

            let text_size = *debug_text_sizes.iter().max().unwrap() as f32;

            // Draw besides the asteroid
            let x_offset = if screen_width() - position.x >= text_size + 25.0 {
                25.0 * self.size as f32
            } else {
                -text_size + 25.0
            };
            let y_offset = if screen_height() - position.y >= font_size * texts.len() as f32 {
                20.0
            } else {
                -font_size * texts.len() as f32
            };

            // Hitbox
            draw_circle_lines(
                position.x,
                position.y,
                20.0 * self.size as f32 - 3.0,
                3.0,
                Color::from_hex(0x0000FF),
            );
            // Center
            draw_circle_lines(position.x, position.y, 3.0, 1.5, Color::from_hex(0x0000FF));

            for (index, field) in &mut texts.iter_mut().enumerate() {
                draw_text(
                    field,
                    position.x + x_offset,
                    (position.y + y_offset) + index as f32 * 20.0,
                    font_size,
                    GREEN,
                );
            }

            self.draw_trajectory();
            // Comparison line
            draw_line(
                self.position.x,
                self.position.y,
                self.position.x,
                self.position.y - 75.0,
                1.0,
                WHITE,
            );
        }
    }
}

// Initialize the textures lazily at runtime using lazy_static
// Comparable to a constant but more flexible
lazy_static! {
    pub static ref TEXTURES: Vec<String> = get_textures("./assets/textures/asteroid/");
}
