use colored::Colorize;

use crate::{aco::{config::ACOConfig, logger::ACOLogger}, conformation::Conformation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultLogger {
    Iteration,
    Ant,
    None
}

impl ACOLogger for DefaultLogger {
    fn log_iteration(&self, config: ACOConfig, iter: u16, _: &Conformation, fit: f64) {
        match self {
            DefaultLogger::Iteration | DefaultLogger::Ant => {
                println!("{}", "=".repeat(93).blue().bold());
                println!(
                    "{} {:0>2}/{:0>2}  |  Melhor Fitness: {}  |  {} ants  |  Evaporação: {:.2}  |  α: {:.2} β: {:.2}",
                    " ITERAÇÃO:".bold().blue(),
                    (iter + 1).to_string().bold().yellow(),
                    config.max_iter.to_string().bold().yellow(),
                    fit.to_string().green().bold(),
                    config.ant_count.to_string().bold().cyan(),
                    config.evaporation,
                    config.alpha,
                    config.beta
                );
                println!("{}", "=".repeat(93).blue().bold());
            }
            _ => {}
        }
    }

    fn log_ant(&self, _: ACOConfig, ant: u16, _: &Conformation, fit: f64) {
        if let DefaultLogger::Ant = self {
            println!(
                " {}: {:0>2}  |  Fitness: {}",
                "Ant".magenta(),
                ant.to_string().yellow().bold(),
                fit.to_string().green().bold()
            );
        }
    }
}