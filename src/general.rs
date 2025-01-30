use crate::asteroid::Asteroid;
use crate::missile::Missile;
use crate::spaceship::Spaceship;

pub struct Gamestate {
    pub delta_time: f64,
    pub simulation_speed: f64,
    pub debug: bool,
    pub loop_number: u128,
    pub asteroids: Vec<Asteroid>,
    pub missiles: Vec<Missile>,
    pub spaceship: Spaceship,
    pub asteroid_limit: usize,
    pub number_of_asteroids: usize,
    pub lives: u8,
    pub score: u128,
    pub game_started: bool,
    pub game_over: bool,
    pub game_won: bool,
}

impl Gamestate {
    pub fn new() -> Gamestate {
        Gamestate {
            delta_time: 0.0,
            simulation_speed: 1.0,
            debug: false,
            loop_number: 0,
            asteroids: Vec::new(),
            missiles: Vec::new(),
            spaceship: Spaceship::new(),
            asteroid_limit: 26,
            number_of_asteroids: 0,
            lives: 3,
            score: 0,
            game_started: false,
            game_over: false,
            game_won: false,
        }
    }
}