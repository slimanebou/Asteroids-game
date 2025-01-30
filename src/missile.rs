use macroquad::prelude::*;

pub struct Missile {
    pub position: Vec2,
    pub velocity: f32,
    pub rotation: f32,
    pub active: bool,
    pub size: f32,
}

impl Missile {
    /// Crée un nouveau missile à une position donnée avec une vitesse donnée.
    pub fn new(position: Vec2, direction: f32, rotation: f32) -> Self {
        Self {
            position,
            velocity: direction.abs(),
            rotation,
            active: true,
            size: 5.0,
        }
    }

    /// Met à jour la position du missile. Désactive le missile s'il sort de l'écran.
    pub fn update(&mut self, delta_time: f64) {
        //cos pour x, sin pour y
        let direction_vec = vec2(self.rotation.cos(), self.rotation.sin());
        self.position += direction_vec * (self.velocity + 200.0) * delta_time as f32;
        if self.position.x < 0.0
            || self.position.x > screen_width()
            || self.position.y < 0.0
            || self.position.y > screen_height()
        {
            self.active = false; // Désactive le missile hors de l'écran
        }
    }

    /// Affiche le missile si actif.
    pub fn draw(&self) {
        if self.active {
            draw_circle(self.position.x, self.position.y, self.size, RED); // Dessine un petit cercle rouge
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missile_creation() {
        let position = vec2(100.0, 100.0);
        let direction = 1.0;
        let rotation = 0.0;
        let missile = Missile::new(position, direction, rotation);

        assert_eq!(missile.position, position);
        assert_eq!(missile.velocity, direction.abs());
        assert_eq!(missile.rotation, rotation);
        assert!(missile.active);
        assert_eq!(missile.size, 5.0);
    }

    #[test]
    fn test_missile_draw() {
        let missile = Missile::new(vec2(100.0, 100.0), 1.0, 0.0);
        assert!(missile.active);
    }
}
