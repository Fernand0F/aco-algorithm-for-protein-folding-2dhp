use crate::{aco::config::ACOConfig, conformation::{Conformation, Direction}, protein::{AminoAcid, Protein}};

#[derive(Debug, PartialEq, Clone)]
pub struct Pheromones {
    pheromones: Vec<Vec<f64>>,
    config: ACOConfig,
    h_count: f64
}

impl Pheromones {
    pub fn new(protein: &Protein, config: ACOConfig) -> Self {
        let n = protein.len() - 2;

        let h_count = protein.iter()
            .filter(|&&aa| aa == AminoAcid::Hydrophobic)
            .count() as f64;

        Self {
            pheromones: vec![vec![0.5; 3]; n],
            config,
            h_count
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
    
                    let delta = if dir == conf.get(i).unwrap() {
                        fitness / self.h_count
                    } else {
                        0.0
                    };

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