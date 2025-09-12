use std::collections::{HashMap, HashSet};
use rand::{distr::{weighted::WeightedIndex, Distribution}, rng, rngs::ThreadRng, seq::IndexedRandom};

use crate::{aco::config::ACOConfig, pheromones::Pheromones, protein::{AminoAcid, Protein}};

pub mod display;
pub mod local_search;

#[derive(Debug, Clone)]
pub struct Conformation<'a> {
    protein: &'a Protein,
    conformation: Vec<Option<Direction>>,
    config: ACOConfig,
    i: usize,
    rng: ThreadRng
}

impl<'a> Conformation<'a> {
    pub fn new(protein: &'a Protein, config: ACOConfig) -> Self {
        Self {
            protein,
            conformation: vec![None; protein.len() - 2],
            config,
            i: 0,
            rng: rng()
        }
    }

    pub fn get(&self, i: usize) -> Option<Direction> {
        self.conformation[i]
    }

    pub fn is_fully_grown(&self) -> bool {
        self.i == self.conformation.len()
    }

    pub fn grow(&mut self, pheromones: &Pheromones) -> bool {
        if self.is_fully_grown() {
            return true;
        }

        let original_direction = self.conformation[self.i]; /* Salva direção para resetar em caso de erro */
        
        let fitness = self.evaluate();

        let valid_directions: Vec<Direction> = Direction::iter()
            .filter(|d| {
                self.conformation[self.i] = Some(*d);
                self.is_valid()
            })
            .collect();

        if valid_directions.is_empty() {
            self.conformation[self.i] = original_direction;
            return false;
        }

        let weights: Vec<f64> = valid_directions.iter().map(|&d| {
                self.conformation[self.i] = Some(d);
                let new_fitness = self.evaluate();
                let h = new_fitness - fitness + 1.0;

                pheromones.get_weight(self.i, d, h)
            })
            .collect();
                
        self.conformation[self.i] = if weights.iter().sum::<f64>() == 0.0 {
            Some(*valid_directions.choose(&mut self.rng).unwrap())
        } else {
            let dist = WeightedIndex::new(&weights).unwrap();
            Some(valid_directions[dist.sample(&mut self.rng)])
        };

        self.i += 1;

        true
    }

    pub fn rewind(&mut self) {
        self.i /= 2;

        for i in self.i..self.conformation.len() {
            if self.conformation[i].is_none() {
                break;
            }
            self.conformation[i] = None;
        }
    }

    pub fn is_valid(&self) -> bool {
        let mut filled = HashSet::from([(0, 0), (1, 0)]);
        
        let mut pos = (1, 0);
        let mut v = (1, 0);

        let n = self.conformation.len();

        for (i, direction) in self.conformation.iter().enumerate() {
            if let Some(direction) = direction {
                v = Conformation::get_new_velocity(v, *direction);
            }

            pos.0 += v.0;
            pos.1 += v.1;

            let is_dead_end = i > 0 && i < n && [(1, 0), (-1, 0), (0, 1), (0, -1)].iter()
                .all(|offset| {
                    let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
                    filled.contains(&new_pos)
                });

            if filled.contains(&pos) || is_dead_end {
                return false;
            }

            filled.insert(pos);
        }
        
        true
    }

    pub fn evaluate(&self) -> f64 {
        let mut fold = HashMap::new();
        
        // Marca aminoácidos fixos
        fold.insert((0, 0), 0);
        fold.insert((1, 0), 1);
        
        let mut pos = (1, 0); /* Inicializa posição do agente */
        let mut v = (1, 0);   /* Inicializa velocidade do agente */

        for (i, direction) in self.conformation.iter().enumerate() {
            if let Some(direction) = direction {
                v = Conformation::get_new_velocity(v, *direction); /* Calcula nova velocidade */
                pos = (pos.0 + v.0, pos.1 + v.1); /* Aplica nova velocidade */
    
                fold.insert(pos, i + 2); /* Marca posição do aminoácido */
            }
        }

        let mut hh_count = 0;
        for (&pos, &i) in fold.iter() {
            if self.protein[i] == AminoAcid::Hydrophobic {
                let neighbour_offsets = [(1, 0), (-1, 0), (0, 1), (0, -1)];

                for offset in neighbour_offsets {
                    let neighbour_pos = (pos.0 + offset.0, pos.1 + offset.1);
                    
                    if let Some(&neighbour_idx) = fold.get(&neighbour_pos) {
                        if self.protein[neighbour_idx] == AminoAcid::Hydrophobic
                        && (neighbour_idx as isize - i as isize).abs() > 1 {
                            hh_count += 1;
                        }
                    }
                }
            }
        }

        (hh_count / 2) as f64
    }

    fn get_new_velocity(v: (i32, i32), direction: Direction) -> (i32, i32) {
        match direction {
            Direction::Left => match v {
                (1,  0) => (0,  1),
                (-1, 0) => (0, -1),
                (0,  1) => (-1, 0),
                (0, -1) => (1,  0),
                _ => panic!("Velocidade inválida"),
            },
            Direction::Right => match v {
                (1,  0) => (0, -1),
                (-1, 0) => (0,  1),
                (0,  1) => (1,  0),
                (0, -1) => (-1, 0),
                _ => panic!("Velocidade inválida"),
            },
            _ => v,
        }
    }

    pub fn to_string(&self) -> String {
        self.conformation.iter()
            .map(|dir| dir.unwrap().char())
            .collect()
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Straight,
    Left,
    Right
}

impl Direction {
    pub fn as_index(&self) -> usize {
        match self {
            Direction::Straight => 0,
            Direction::Left => 1,
            Direction::Right => 2
        }
    }

    pub fn iter() -> impl Iterator<Item = Direction> {
        [Direction::Straight, Direction::Left, Direction::Right].into_iter()
    }

    pub fn vec() -> Vec<Direction> {
        Self::iter().collect()
    }

    pub fn char(&self) -> char {
        match self {
            Direction::Straight => 'S',
            Direction::Left => 'L',
            Direction::Right => 'R'
        }
    }
}