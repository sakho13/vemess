/// Bohr 半径での長さ
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bohr(pub f64);

impl Bohr {
    /// オングストロームへ変換
    pub fn to_angstrom(self) -> Angstrom {
        Angstrom(self.0 * 0.529177)
    }
}

/// オングストローム単位の長さ
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angstrom(pub f64);

impl Angstrom {
    /// Bohr へ変換
    pub fn to_bohr(self) -> Bohr {
        Bohr(self.0 / 0.529177)
    }
}
