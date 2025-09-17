use macroquad::{shapes::draw_rectangle, window::{screen_height, screen_width}};

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
            pheromones: vec![vec![0.3; 3]; n],
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
    
    pub fn draw(&self) {
        let width = screen_width();
        let height = screen_height();
        
        let cell_w = width / self.pheromones.len() as f32;
        let cell_h = 20.0; // altura da barra de feromônio
        let base_y = height - cell_h * 3.0 - 10.0; // margem inferior

        // Normaliza todos os valores para [0, 1]
        let max_val = self.pheromones
            .iter()
            .flat_map(|v| v.iter())
            .cloned()
            .fold(0.0/0.0, f64::max) // pega o máximo
            .max(1e-6); // evita divisão por zero

        for (i, line) in self.pheromones.iter().enumerate() {
            for (d, &val) in line.iter().enumerate() {
                let norm = (val / max_val) as f32;
                
                // Interpolação entre azul (baixo) e vermelho (alto)
                let color = macroquad::color::Color {
                    r: norm,
                    g: 0.0,
                    b: 1.0 - norm,
                    a: 1.0
                };

                // Cada direção ocupa uma "faixa" dentro da coluna
                let x = i as f32 * cell_w;
                let y = base_y + d as f32 * (cell_h + 2.0);
                
                draw_rectangle(x, y, cell_w - 2.0, cell_h, color);
            }
        }
    }
}