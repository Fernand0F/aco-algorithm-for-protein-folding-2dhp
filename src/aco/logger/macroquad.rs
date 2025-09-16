use crate::{aco::{config::ACOConfig, logger::AsyncACOLogger}, conformation::Conformation};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MacroquadLogger;

impl AsyncACOLogger for MacroquadLogger {
    async fn log_iteration(&self, _: ACOConfig, iter: u16, conformation: &Conformation, fit: f64) {
        conformation.draw(iter, fit).await;
    }

    async fn log_ant(&self, _: ACOConfig, iter: u16, conformation: &Conformation, fit: f64) {
        conformation.draw(iter, fit).await;
    }

    async fn log_change(&self, _: ACOConfig, iter: u16, conformation: &Conformation, fit: f64) {
        conformation.draw(iter, fit).await;
    }
}