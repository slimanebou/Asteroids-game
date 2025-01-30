use macroquad::prelude::*;
use std::f32::consts::PI;

// Définir la structure du vaisseau
pub struct Spaceship {
    pub position: Vec2,
    pub speed: f32,
    pub max_speed: f32,
    pub rotation: f32,
    pub turn_rate: f32,
    pub size: f32,
}

impl Spaceship {
    // Méthode pour créer une nouvelle instance du vaisseau
    pub fn new() -> Self {
        Spaceship {
            // position: vec2(100.0, 100.0), //for test
            position: vec2(screen_width() / 2.0, screen_height() / 2.0),
            speed: 0.0,
            max_speed: 500.0,
            rotation: 0.0,  // Orientation actuelle
            turn_rate: 4.0, // Vitesse de rotation
            size: 20.0,
        }
    }

    // Rotate a point based on the rotation
    fn rotate_point(&self, point: Vec2, rotation_angle: f32) -> Vec2 {
        let cos_angle = rotation_angle.cos();
        let sin_angle = rotation_angle.sin();

        Vec2::new(
            point.x * cos_angle - point.y * sin_angle,
            point.x * sin_angle + point.y * cos_angle,
        )
    }

    // Méthodes pour dessiner le vaisseau sous forme de triangle
    pub fn draw(&mut self, size: f32, debug: bool) {
        // Calculate the height of the equilateral triangle
        let height = size * (PI / 3.0).cos();
        let position = self.position;

        // Define the three points for the equilateral triangle
        // Front point pointing in the direction of the spaceship
        let front = Vec2::new(size, 0.0);
        let left = Vec2::new(-size / 2.0, height);
        let right = Vec2::new(-size / 2.0, -height);

        // Rotate the points based on the spaceship's rotation
        let rotated_front = self.rotate_point(front, self.rotation);
        let rotated_left = self.rotate_point(left, self.rotation);
        let rotated_right = self.rotate_point(right, self.rotation);

        // Draw the triangle with rotated points, centered at position
        draw_triangle(
            self.position + rotated_front,
            self.position + rotated_left,
            self.position + rotated_right,
            YELLOW,
        );

        // statistiques
        if debug {
            // Hitbox
            draw_circle_lines(
                position.x,
                position.y,
                self.size - 3.0,
                3.0,
                Color::from_hex(0x0000FF),
            );

            let font_size = 20.0;
            // Attributes
            let mut texts = Vec::from([
                format!("x: {:.2} y: {:.2}", position.x, position.y),
                format!("Velocity:{:.2}px/s", self.speed),
                format!("Rotation:{:.2}rad", self.rotation),
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

            // Draw besides the asteroid and not out of bounds
            let x_offset = if screen_width() - position.x >= text_size + 25.0 {
                25.0
            } else {
                -text_size + 25.0
            };
            let y_offset = if screen_height() - position.y >= font_size * texts.len() as f32 {
                20.0
            } else {
                -font_size * texts.len() as f32
            };

            for (index, field) in &mut texts.iter_mut().enumerate() {
                draw_text(
                    field,
                    position.x + x_offset,
                    (position.y + y_offset) + index as f32 * 20.0,
                    font_size,
                    PINK,
                );
            }
        }
    }

    // Méthode pour mettre à jour la position du vaisseau
    pub fn update(&mut self, delta_time: f64) {
        // Calculate velocity based on rotation and max speed
        let direction = vec2(self.rotation.cos(), self.rotation.sin());

        // Prevent the spaceship from going faster than the max speed
        if self.speed > self.max_speed {
            self.speed = self.max_speed
        }
        // Update position using the current speed and direction
        self.position += direction * self.speed * delta_time as f32;

        // Handle screen wrapping (loop the spaceship)
        if self.position.x < 0.0 {
            self.position.x = screen_width();
        } else if self.position.x > screen_width() {
            self.position.x = 0.0;
        }

        if self.position.y < 0.0 {
            self.position.y = screen_height();
        } else if self.position.y > screen_height() {
            self.position.y = 0.0;
        }
    }

    /// Moves the object by adding rotation within the bounds of [-2PI;2PI]
    pub fn add_rotation(&mut self, amount: f32) {
        self.rotation = (self.rotation + amount) % (PI * 2.0);
    }

    /// Moves the object based on its speed, with greater force on opposing direction.
    pub fn move_spaceship(&mut self, delta_time: f64, movement_type: bool) {
        let movement_direction = if movement_type { 1.0 } else { -1.0 };
        // The spaceship goes in the opposite direction faster
        let acceleration_factor = if self.speed.signum() == -movement_direction {
            3.0
        } else {
            1.0
        };
        let base_acceleration = 150.0;
        let acceleration =
            base_acceleration * movement_direction * acceleration_factor * delta_time as f32;

        // Accelerate if it would not go over the max speed attribute
        if (self.speed + acceleration).abs() < self.max_speed {
            self.speed += acceleration;
        // Cap the speed to the max speed attribute
        } else {
            self.speed = self.max_speed * movement_direction;
        }
    }

    /// Returns the collision_radius
    pub fn get_collision_radius(&self) -> f32 {
        // la distance maximale du centre du triangle au sommet
        self.size / (2.0 * (PI / 3.0).cos())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spaceship_creation() {
        let spaceship = Spaceship::new();

        // Vérifie que la position du vaisseau au (100.0, 100.0)
        assert_eq!(spaceship.position.x, 100.0);
        assert_eq!(spaceship.position.y, 100.0);

        // Vérifie que la vitesse initiale du vaisseau est 0
        assert_eq!(spaceship.speed, 0.0);

        // Vérifie que la rotation initiale est 0
        assert_eq!(spaceship.rotation, 0.0);
    }

    #[test]
    fn test_spaceship_rotation() {
        let mut spaceship = Spaceship::new();

        // Effectuer une rotation de 90° (PI / 2)
        spaceship.add_rotation(PI / 2.0);

        // Vérifie que la rotation est correcte
        assert_eq!(spaceship.rotation, PI / 2.0);
    }

    //test avancer tout droit
    #[test]
    fn test_spaceship_move_forward() {
        let mut spaceship = Spaceship::new();

        // Définit la vitesse du vaisseau et l'accélère
        spaceship.speed = 100.0;
        spaceship.move_spaceship(1.0, true); // Fait avancer le vaisseau

        // Vérifie que la vitesse a été modifiée
        assert!(
            spaceship.speed > 100.0,
            "La vitesse devrait avoir augmenté."
        );
    }

    //test reculer le vaisseau
    #[test]
    fn test_spaceship_move_backward() {
        let mut spaceship = Spaceship::new();

        // Définit la vitesse du vaisseau et inverse le mouvement
        spaceship.speed = 100.0;
        spaceship.move_spaceship(1.0, false); // Fait reculer le vaisseau

        // Vérifie que la vitesse a été modifiée
        assert!(spaceship.speed < 100.0, "La vitesse devrait avoir diminué.");
    }

    #[test]
    fn test_spaceship_max_speed() {
        let mut spaceship = Spaceship::new();

        // Accélère jusqu'à dépasser la vitesse maximale
        spaceship.speed = 600.0; // Au-dessus de la vitesse maximale
        spaceship.move_spaceship(1.0, true); // Essaye de faire avancer le vaisseau

        // Vérifie que la vitesse ne dépasse pas la vitesse maximale
        assert_eq!(
            spaceship.speed, spaceship.max_speed,
            "La vitesse ne doit pas dépasser la vitesse maximale."
        );
    }
}
