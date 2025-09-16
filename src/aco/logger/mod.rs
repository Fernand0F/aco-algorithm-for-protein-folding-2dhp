use crate::{aco::config::ACOConfig, conformation::Conformation};

pub mod default;
pub mod macroquad;

pub trait ACOLogger: Clone + Copy {
    fn log_iteration(&self, config: ACOConfig, iter: u16, conformation: &Conformation, fit: f64);
    fn log_ant(&self, config: ACOConfig, ant: u16, conformation: &Conformation, fit: f64);
}

pub trait AsyncACOLogger: Clone + Copy {
    async fn log_iteration(&self, config: ACOConfig, iter: u16, conformation: &Conformation, fit: f64);
    async fn log_ant(&self, config: ACOConfig, iter: u16, conformation: &Conformation, fit: f64);
    async fn log_change(&self, config: ACOConfig, iter: u16, conformation: &Conformation, fit: f64);
}