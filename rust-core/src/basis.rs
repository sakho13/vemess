use crate::molecule::AtomicNumber;
use crate::units::Bohr;
use std::collections::HashMap;

/// 角運動量 (l, m, n) を表す構造体
#[derive(Debug, Clone, Copy)]
pub struct AngularMomentum {
    pub l: u8,
    pub m: u8,
    pub n: u8,
}

impl AngularMomentum {
    /// 新しい角運動量を作成する
    pub fn new(l: u8, m: u8, n: u8) -> Self {
        Self { l, m, n }
    }
}

/// 基本ガウス関数
#[derive(Debug, Clone)]
pub struct PrimitiveGaussian {
    /// 指数
    pub exponent: f64,
    /// 収縮係数
    pub coefficient: f64,
    /// 中心座標
    pub center: [Bohr; 3],
    /// 角運動量
    pub angular: AngularMomentum,
}

/// 複数の基本ガウス関数からなる収束ガウス
#[derive(Debug, Clone)]
pub struct ContractedGaussian {
    pub primitives: Vec<PrimitiveGaussian>,
}

impl ContractedGaussian {
    /// 各プリミティブの指数が正か確認する
    pub fn validate(&self) -> bool {
        self.primitives.iter().all(|p| p.exponent > 0.0)
    }
}

/// 原子番号ごとの基底関数集合
#[derive(Debug, Default)]
pub struct BasisSet {
    map: HashMap<AtomicNumber, Vec<ContractedGaussian>>,
}

impl BasisSet {
    /// 指定した原子番号に基底を追加
    pub fn insert(&mut self, z: AtomicNumber, cg: ContractedGaussian) {
        self.map.entry(z).or_default().push(cg);
    }

    /// 基底を取得
    pub fn get(&self, z: AtomicNumber) -> Option<&[ContractedGaussian]> {
        self.map.get(&z).map(|v| v.as_slice())
    }

    /// すべての基底が有効か検証
    pub fn validate(&self) -> bool {
        self.map.values().flatten().all(|cg| cg.validate())
    }
}
