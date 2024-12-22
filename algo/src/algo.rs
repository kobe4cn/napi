use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Algo {
    pub algo_type: AlgoType,
}
#[derive(Debug, Clone)]
pub enum AlgoType {
    Blake3,
    Default,
}
impl AlgoType {
    pub fn as_str(&self) -> &str {
        match self {
            AlgoType::Blake3 => "blake3",
            AlgoType::Default => "default",
        }
    }
}

impl Algo {
    pub fn new(algo_type: AlgoType) -> Self {
        Self { algo_type }
    }

    pub fn hash(&self, v: String) -> String {
        match self.algo_type {
            AlgoType::Blake3 => {
                let hash = blake3::hash(v.as_bytes());
                hash.to_string()
            }
            AlgoType::Default => {
                let mut hasher = DefaultHasher::new();
                v.hash(&mut hasher);
                hasher.finish().to_string()
            }
        }
    }

    pub fn get_name(&self) -> &str {
        self.algo_type.as_str()
    }
}
