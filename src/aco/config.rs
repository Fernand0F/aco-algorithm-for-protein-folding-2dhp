#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ACOConfig {
    pub ant_count: u16,
    pub max_iter: u16,
    pub no_impr_max: u16,
    pub evaporation: f64,
    pub alpha: f64,
    pub beta: f64,
    pub neutral_mutation_rate: f64
}