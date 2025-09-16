use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

use crate::conformation::{Conformation, Direction};

impl Conformation {
    pub fn local_search(&mut self, rng: &mut ThreadRng) -> bool {
        let mut improved = false;

        improved = self.point_mutation(rng) || improved;
        improved = self.macro_mutation_neightbourhood(rng) || improved;
        
        improved
    }

    fn point_mutation(&mut self, rng: &mut ThreadRng) -> bool {
        let mut indexes: Vec<usize> = (0..self.conformation.len()).collect();
        indexes.shuffle(rng); /* Embraralha os índices */

        let mut current_fitness = self.eval(); /* Pega fitness para comparação */
        let mut improved = false; /* Variável apra verificar se houve melhora */

        let directions: Vec<Direction> = Direction::vec();

        for i in indexes {
            let mut original_direction = self.conformation[i]; /* Salva direção original para reverção */

            for &d in &directions {
                self.conformation[i] = Some(d); /* Tenta uma nova direção */

                if !self.is_valid() { /* Verica se gera sobreposição */
                    self.conformation[i] = original_direction; /* Reverte se for inválido */
                    continue;
                }

                let new_fitness = self.eval(); /* Faz nova avaliação */

                if new_fitness > current_fitness {
                    current_fitness = new_fitness;
                    original_direction = self.conformation[i];
                    improved = true;
                } else if new_fitness == current_fitness && rng.random::<f64>() <= self.config.neutral_mutation_rate {
                    original_direction = self.conformation[i];
                } else {
                    self.conformation[i] = original_direction; /* Reverte se não melhorar */
                }
            }
        }
        
        improved
    }

    fn macro_mutation_neightbourhood(&mut self, rng: &mut ThreadRng) -> bool {
        let backup = self.conformation.clone();
        let fitness = self.eval();

        let n = self.conformation.len();

        let mut i = rng.random_range(0..n);
        let mut j = rng.random_range(0..n);

        if j < i {
            let temp = i;
            i = j;
            j = temp;
        }

        for k in i..=j {
            let original = self.conformation[k].unwrap();

            let mut new_directions: Vec<Direction> = Direction::iter().filter(|d| *d != original).collect();
            new_directions.shuffle(rng);

            self.conformation[k] = Some(new_directions[0]);

            if !self.is_valid() {
                self.conformation[k] = Some(new_directions[0]);
                
                if !self.is_valid() {
                    self.conformation[k] = Some(original);
                }
            }
        }

        let new_fitness = self.eval(); 

        if new_fitness > fitness {
            true
        } else if new_fitness == fitness {
            if rng.random::<f64>() > self.config.neutral_mutation_rate {
                self.conformation = backup;
            }
            false
        } else {
            self.conformation = backup;
            false
        }
    }
}