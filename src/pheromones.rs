use crate::{aco::config::ACOConfig, conformation::{Conformation, Direction}};

#[derive(Debug, PartialEq, Clone)]
pub struct Pheromones {
    pheromones: Vec<Vec<f64>>,
    config: ACOConfig
}

impl Pheromones {
    pub fn new(n: usize, config: ACOConfig) -> Self {
        Self {
            pheromones: vec![vec![0.5; 3]; n],
            config
        }
    }

    pub fn get_weight(&self, i: usize, d: Direction, h: f64) -> f64 {
        self.pheromones[i][d.as_index()].powf(self.config.alpha) * h.powf(self.config.beta)
    }

    pub fn update(&mut self, conformations: &[Conformation]) {
        self.evaporate();

        for conf in conformations {
            let fitness = conf.eval();
    
            let directions = Direction::vec();
    
            for (i, line) in self.pheromones.iter_mut().enumerate() {
                for didx in 0..line.len() {
                    let dir = directions[didx];
    
                    let delta = if dir == conf.get(i).unwrap() { fitness } else { 0.0 };
    
                    line[didx] += delta;
                }
            }
        }
    }

    fn evaporate(&mut self) {
        for line in self.pheromones.iter_mut() {
            for didx in 0..line.len() {
                line[didx] *= 1.0 - self.config.evaporation;
            }
        }
    }
}