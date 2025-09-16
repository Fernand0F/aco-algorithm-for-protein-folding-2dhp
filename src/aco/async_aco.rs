use rand::{rng, rngs::ThreadRng};

use crate::{aco::{config::ACOConfig, logger::AsyncACOLogger}, conformation::Conformation, pheromones::Pheromones, protein::Protein};

pub async fn async_aco_protein_folding_2dhp(
    protein: &Protein,
    config: ACOConfig,
    logger: impl AsyncACOLogger
) -> (Conformation, f64)
{
    let n = protein.len();
    let mut pheromones = Pheromones::new(n - 2, config);

    let mut best_conformation = Conformation::new(protein, config);
    let mut best = f64::NEG_INFINITY;

    for iteration in 0..config.max_iter {        
        let mut conformations_zip = Vec::new();
        
        for ant in 0..config.ant_count {
            let mut conf = Conformation::new(protein, config); /* Cria nova conformação */
            let mut rng = rng();

            while !conf.is_fully_grown() {
                if !conf.grow(&pheromones, &mut rng) {
                    conf.rewind();
                }
                // logger.log_change(config, iteration, &conf, best).await;
            }
            
            // Tenta melhorar solução encontrada
            local_search_loop(&mut conf, config.no_impr_max, &mut rng, logger).await;

            let fit = conf.eval(); /* Avalia para comparação */

            logger.log_ant(config, iteration, &conf, best).await;

            conformations_zip.push((conf, fit));
        };

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

        logger.log_iteration(config, iteration, &best_conformation, best).await;
    }

    (best_conformation, best)
}

async fn local_search_loop(
    conformation: &mut Conformation,
    no_impr_max: u16,
    rng: &mut ThreadRng,
    _: impl AsyncACOLogger
) {
    let mut no_impr = 0;
    while no_impr < no_impr_max {
        if conformation.local_search(rng) {
            no_impr = 0;
        } else {
            no_impr += 1;
        }
        // logger.log_change(config, iter, conformation, fit)
    }
}