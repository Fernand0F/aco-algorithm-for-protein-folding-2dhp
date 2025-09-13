use std::{fs::{self, OpenOptions}, io::Write, time::Instant};

use crate::{aco::{aco_protein_folding_2dhp, config::ACOConfig, DisplayType}, protein::AminoAcid};
use macroquad::prelude::*;

mod aco;
mod protein;
mod pheromones;
mod conformation;

#[macroquad::main(window_conf)]
async fn main() {
    let max_iter = 30;

    let config = ACOConfig {
        ant_count: 10,
        max_iter,
        no_impr_max: 10,
        evaporation: 0.9,
        alpha: 2.0,
        beta: 1.0,
        neutral_mutation_rate: 0.5
    };

    let (protein, _) = load_benchmark(7);

    let display = DisplayType::Ant;

    loop {
        let start = Instant::now();
        let (conformation, best_found) = aco_protein_folding_2dhp(&protein, config, display).await;
        let duration = start.elapsed();
        println!("Tempo: {:?}", duration);

        loop {
            conformation.draw(config.max_iter, best_found).await;
            
            if is_key_pressed(KeyCode::Enter) {
                break;
            }
        }
    }
}

// #[macroquad::main(window_conf)]
// fn main() {
//     let ant_count = [10, 20];
//     let no_impr_max = [10, 20];
    
//     // Fernando evaporação = 0.3
//     // Ku evaporação = 0.6
//     // Gava evaporação = 0.9
//     let evaporation = [0.3];

//     let alpha = [1.0, 2.0, 3.0];
//     let beta = [1.0, 2.0, 3.0];
//     let neutral_mutation_rate = [0.0, 0.5];

//     for _ in 0..100 {
//         for ac in ant_count {
//             for nim in no_impr_max {
//                 for e in evaporation {
//                     for a in alpha {
//                         for b in beta {
//                             for n in neutral_mutation_rate {
//                                 let config = ACOConfig {
//                                     ant_count: ac,
//                                     max_iter: 75, /* fixo */
//                                     no_impr_max: nim,
//                                     evaporation: e,
//                                     alpha: a,
//                                     beta: b,
//                                     neutral_mutation_rate: n
//                                 };
    
//                                 for i in 0..9 {
//                                     run_benchmark(i, config);
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }    
//         }
//     }
// }

async fn run_benchmark(i: usize, config: ACOConfig) {
    let (protein, _) = load_benchmark(i);

    let (conformation, best_found) = aco_protein_folding_2dhp(&protein, config, DisplayType::None).await;

    let benchmark = format!("{}:{}:{}:{}:{}:{}:{}:{}:{}\n",
        i,
        config.ant_count,
        config.no_impr_max,
        config.evaporation,
        config.alpha,
        config.beta,
        config.neutral_mutation_rate,
        conformation.to_string(),
        best_found
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("benchmark_results.txt")
        .expect("Não foi possível abrir o arquivo");

    file.write_all(benchmark.as_bytes())
        .expect("Erro ao escrever no arquivo");
}

fn load_benchmark(i: usize) -> (Vec<AminoAcid>, i32) {
    let file_content = fs::read_to_string("benchmarks.txt").expect("Erro ao ler o arquivo");

    let lines: Vec<&str> = file_content.split("\n").collect();
    let items: Vec<&str> = lines[i].split(":").collect();

    (parse_protein_sequence(items[1]).unwrap(), items[0].parse().unwrap())
}

fn parse_protein_sequence(s: &str) -> Result<Vec<AminoAcid>, String> {
    let mut protein_vec = Vec::new();

    for c in s.chars() {
        match c {
            'H' => protein_vec.push(AminoAcid::Hydrophobic),
            'P' => protein_vec.push(AminoAcid::Polar),
            _ => return Err(format!("Caractere inválido na sequência: '{}'", c)),
        }
    }

    Ok(protein_vec)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Protein Folding 2DHP".to_string(),
        window_width: 600,
        window_height: 600,
        ..Default::default()
    }
}