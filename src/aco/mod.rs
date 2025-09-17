use std::sync::{Arc, Mutex};

use crate::{aco::{config::ACOConfig, logger::ACOLogger}, conformation::Conformation, pheromones::Pheromones, protein::Protein};
use rand::{rng, rngs::ThreadRng};
use rayon::prelude::*;

pub mod config;
pub mod logger;
pub mod async_aco;

pub fn aco_protein_folding_2dhp<L>(
    protein: &Protein,
    config: ACOConfig,
    logger: L
) -> (Conformation, f64)
where
    L: ACOLogger + Send + Sync + 'static,
{
    let mut pheromones = Pheromones::new(&protein, config);

    let mut best_conformation = Conformation::new(protein, config);
    let mut best = f64::NEG_INFINITY;

    let thread_logger = Arc::new(Mutex::new(logger));

    for iteration in 0..config.max_iter {        
        // let conformations_zip: Vec<_> = (0..config.ant_count).into_iter()
        let conformations_zip: Vec<_> = (0..config.ant_count).into_par_iter()
            .map(|ant| {
                let mut conf = Conformation::new(protein, config); /* Cria nova conformação */
                let mut rng = rng();

                while !conf.is_fully_grown() {
                    if !conf.grow(&pheromones, &mut rng) {
                        conf.rewind();
                    }
                }
                
                // Tenta melhorar solução encontrada
                local_search_loop(&mut conf, config.no_impr_max, &mut rng);
    
                let fit = conf.eval(); /* Avalia para comparação */

                thread_logger.lock().unwrap().log_ant(config, ant, &conf, fit);
                                
                (conf, fit)
            }).collect();

        // Atualiza melhor solução
        let conformations: Vec<_> = conformations_zip.into_iter()
            .map(|(conf, fit)| {
                if fit > best {
                    best = fit;
                    best_conformation = conf.clone();
                }
                conf
            })
            .collect();

        pheromones.update(&conformations);

        thread_logger.lock().unwrap().log_iteration(config, iteration, &best_conformation, best);
    }

    (best_conformation, best)
}

fn local_search_loop(conformation: &mut Conformation, no_impr_max: u16, rng: &mut ThreadRng) {
    let mut no_impr = 0;
    while no_impr < no_impr_max {
        if conformation.local_search(rng) {
            no_impr = 0;
        } else {
            no_impr += 1;
        }
    }
}