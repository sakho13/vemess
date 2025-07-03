/// Length in Bohr radii.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bohr(pub f64);

impl Bohr {
    pub fn to_angstrom(self) -> Angstrom {
        Angstrom(self.0 * 0.529177)
    }
}

/// Length in Angstrom.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angstrom(pub f64);

impl Angstrom {
    pub fn to_bohr(self) -> Bohr {
        Bohr(self.0 / 0.529177)
    }
}
