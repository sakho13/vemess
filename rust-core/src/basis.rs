use crate::molecule::AtomicNumber;
use crate::units::Bohr;
use std::collections::HashMap;

/// Primitive Gaussian function.
#[derive(Debug, Clone)]
pub struct PrimitiveGaussian {
    pub exponent: f64,
    pub coefficient: f64,
    pub center: [Bohr; 3],
    pub angular: (u8, u8, u8),
}

/// Contracted Gaussian consisting of multiple primitives.
#[derive(Debug, Clone)]
pub struct ContractedGaussian {
    pub primitives: Vec<PrimitiveGaussian>,
}

impl ContractedGaussian {
    pub fn validate(&self) -> bool {
        self.primitives.iter().all(|p| p.exponent > 0.0)
    }
}

/// Basis set containing contracted Gaussians indexed by atomic number.
#[derive(Debug, Default)]
pub struct BasisSet {
    map: HashMap<AtomicNumber, Vec<ContractedGaussian>>,
}

impl BasisSet {
    /// Insert a basis for a given atomic number.
    pub fn insert(&mut self, z: AtomicNumber, cg: ContractedGaussian) {
        self.map.entry(z).or_default().push(cg);
    }

    /// Retrieve basis for an atom.
    pub fn get(&self, z: AtomicNumber) -> Option<&[ContractedGaussian]> {
        self.map.get(&z).map(|v| v.as_slice())
    }

    /// Validate all primitives.
    pub fn validate(&self) -> bool {
        self.map.values().flatten().all(|cg| cg.validate())
    }
}
