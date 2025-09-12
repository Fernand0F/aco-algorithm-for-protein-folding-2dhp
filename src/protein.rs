#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AminoAcid {
    Polar,
    Hydrophobic
}

pub type Protein = Vec<AminoAcid>;
