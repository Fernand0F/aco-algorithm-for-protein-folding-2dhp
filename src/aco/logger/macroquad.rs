use crate::{aco::{config::ACOConfig, logger::AsyncACOLogger}, conformation::Conformation, pheromones::Pheromones};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MacroquadLogger {
    Iteration,
    Ant,
    Change,
    None
}

impl AsyncACOLogger for MacroquadLogger {
    async fn log_iteration(
        &self,
        _: ACOConfig,
        iter: u16,
        conformation: &Conformation,
        fit: f64,
        pheromones: &Pheromones
    ) {
        if let MacroquadLogger::Iteration = self { 
            conformation.draw(iter, fit, Some(pheromones)).await;
        }
    }

    async fn log_ant(
        &self,
        _: ACOConfig,
        iter: u16,
        conformation: &Conformation,
        fit: f64,
        pheromones: &Pheromones
    ) {
        if let MacroquadLogger::Ant = self { 
            conformation.draw(iter, fit, Some(pheromones)).await;
        }
    }

    async fn log_change(
        &self,
        _: ACOConfig,
        iter: u16,
        conformation: &Conformation,
        fit: f64,
        pheromones: &Pheromones
    ) {
        if let MacroquadLogger::Change = self { 
            conformation.draw(iter, fit, Some(pheromones)).await;
        }
    }
}