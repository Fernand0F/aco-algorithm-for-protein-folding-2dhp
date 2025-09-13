use crate::{aco::config::ACOConfig, conformation::Conformation, pheromones::Pheromones, protein::Protein};
use colored::*;
use rand::{rng, rngs::ThreadRng};
// use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;

pub mod config;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DisplayType {
    Full,
    Ant,
    Iteration,
    None
}

pub async fn aco_protein_folding_2dhp<'a>(
    protein: &'a Protein,
    config: ACOConfig,
    display: DisplayType
) -> (Conformation<'a>, f64) {
    let n = protein.len();
    let mut pheromones = Pheromones::new(n - 2, config);

    let mut best_conformation = Conformation::new(protein, config);
    let mut best = f64::NEG_INFINITY;

    for iteration in 0..config.max_iter {
        println!("\n{}", format!("====== Iteração {:>3}/{:>3} ======", iteration + 1, config.max_iter).bright_blue().bold());
        
        let conformations_zip: Vec<_> = (0..config.ant_count).into_par_iter()
            .map(|_| {
                let mut conf = Conformation::new(protein, config); /* Cria nova conformação */
                let mut rng = rng();

                while !conf.is_fully_grown() {
                    if !conf.grow(&pheromones, &mut rng) {
                        conf.rewind();
                    }
                }
                
                // Tenta melhorar solução encontrada
                local_search_loop(&mut conf, config.no_impr_max, &mut rng);
    
                let fit = conf.evaluate(); /* Avalia para comparação */
                                
                (conf, fit)
            }).collect();

        let conformations: Vec<_> = conformations_zip.into_iter()
            .map(|(conf, fit)| {
                if fit > best {
                    best = fit;
                    best_conformation = conf.clone();
                }
                conf
            })
            .collect();

        if display == DisplayType::Iteration { conformations[0].draw(iteration, best).await }

        pheromones.update(&conformations);
    }

    (best_conformation, best)
}

fn local_search_loop(conformation: &mut Conformation<'_>, no_impr_max: u16, rng: &mut ThreadRng) {
    let mut no_impr = 0;
    while no_impr < no_impr_max {
        if conformation.local_search(rng) {
            no_impr = 0;
        } else {
            no_impr += 1;
        }
    }
}