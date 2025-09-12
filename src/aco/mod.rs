use crate::{aco::config::ACOConfig, conformation::Conformation, pheromones::Pheromones, protein::Protein};
use colored::*;

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

        let mut conformations = Vec::new();
        
        for ant in 0..config.ant_count {
            let mut conf = Conformation::new(protein, config); /* Cria nova conformação */
    
            while !conf.is_fully_grown() {
                if !conf.grow(&pheromones) {
                    conf.rewind();
                }

                if display == DisplayType::Full { conf.draw(iteration, best).await }
            }
            
            // Tenta melhorar solução encontrada
            local_search_loop(&mut conf, config.no_impr_max, display, iteration, best).await;

            let fit = conf.evaluate(); /* Avalia para comparação */
        
            if fit > best {
                best_conformation = conf.clone();
                best = fit
            }

            if display == DisplayType::Ant { conf.draw(iteration, best).await }

            conformations.push(conf);

            println!("Energia formiga {:>2}......: {:.4}", ant, fit.to_string().green());
        }

        if display == DisplayType::Iteration { conformations[0].draw(iteration, best).await }

        pheromones.update(&conformations);

        println!("Melhor energia até agora: {:.4}", best.to_string().cyan());
    }

    (best_conformation, best)
}

async fn local_search_loop(
    conformation: &mut Conformation<'_>,
    no_impr_max: u16,
    display: DisplayType,
    iteration: u16,
    best: f64
) {
    let mut no_impr = 0;
    while no_impr < no_impr_max {
        if conformation.local_search() {
            no_impr = 0;
            if display == DisplayType::Full {
                conformation.draw(iteration, best).await;
            }
        } else {
            no_impr += 1;
        }
    }
}