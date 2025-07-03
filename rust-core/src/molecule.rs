use crate::units::{Angstrom, Bohr};

/// Represents an atomic number (Z).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AtomicNumber(pub u8);

/// Single atom with position information.
#[derive(Debug, Clone)]
pub struct Atom {
    pub atomic_number: AtomicNumber,
    pub position: [Bohr; 3],
}

/// Molecular system consisting of atoms, charge and multiplicity.
#[derive(Debug, Clone)]
pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub total_charge: i32,
    pub multiplicity: u8,
}

impl Molecule {
    /// Create an empty molecule.
    pub fn new(total_charge: i32, multiplicity: u8) -> Self {
        Self { atoms: Vec::new(), total_charge, multiplicity }
    }

    /// Add an atom to the molecule.
    pub fn add_atom(&mut self, atomic_number: AtomicNumber, position: [Bohr; 3]) {
        self.atoms.push(Atom { atomic_number, position });
    }

    /// Remove atom at the specified index.
    pub fn remove_atom(&mut self, index: usize) {
        self.atoms.remove(index);
    }

    /// Returns the geometric center of the molecule in Bohr.
    pub fn center(&self) -> [Bohr; 3] {
        let mut sum = [Bohr(0.0); 3];
        for atom in &self.atoms {
            for i in 0..3 {
                sum[i].0 += atom.position[i].0;
            }
        }
        let n = self.atoms.len() as f64;
        if n == 0.0 {
            return [Bohr(0.0); 3];
        }
        [Bohr(sum[0].0 / n), Bohr(sum[1].0 / n), Bohr(sum[2].0 / n)]
    }

    /// Validates the total charge against atomic numbers.
    pub fn validate_charge(&self) -> bool {
        let z_sum: i32 = self.atoms.iter().map(|a| a.atomic_number.0 as i32).sum();
        z_sum - self.total_charge >= 0
    }

    /// Convert all coordinates to Angstrom.
    pub fn to_angstrom(&self) -> Vec<[Angstrom; 3]> {
        self.atoms
            .iter()
            .map(|a| a.position.map(|b| b.to_angstrom()))
            .collect()
    }
}
