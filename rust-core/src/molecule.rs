use crate::units::{Angstrom, Bohr};

/// 原子番号 (Z) を表す型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AtomicNumber(pub u8);

/// 座標を持つ単一原子
#[derive(Debug, Clone)]
pub struct Atom {
    pub atomic_number: AtomicNumber,
    pub position: [Bohr; 3],
}

/// 複数の原子からなる分子
#[derive(Debug, Clone)]
pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub total_charge: i32,
    pub multiplicity: u8,
}

impl Molecule {
    /// 空の分子を作成
    pub fn new(total_charge: i32, multiplicity: u8) -> Self {
        Self { atoms: Vec::new(), total_charge, multiplicity }
    }

    /// 原子を追加
    pub fn add_atom(&mut self, atomic_number: AtomicNumber, position: [Bohr; 3]) {
        self.atoms.push(Atom { atomic_number, position });
    }

    /// 指定インデックスの原子を削除
    pub fn remove_atom(&mut self, index: usize) {
        self.atoms.remove(index);
    }

    /// 幾何中心を Bohr 単位で返す
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

    /// 電荷整合性を検証
    pub fn validate_charge(&self) -> bool {
        let z_sum: i32 = self.atoms.iter().map(|a| a.atomic_number.0 as i32).sum();
        z_sum - self.total_charge >= 0
    }

    /// すべての座標をオングストロームに変換
    pub fn to_angstrom(&self) -> Vec<[Angstrom; 3]> {
        self.atoms
            .iter()
            .map(|a| a.position.map(|b| b.to_angstrom()))
            .collect()
    }
}
