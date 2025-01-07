use roaring::RoaringBitmap;
#[derive(Debug, Clone)]
pub struct BitMap {
    bitmap: RoaringBitmap,
}

impl BitMap {
    pub fn new() -> Self {
        Self {
            bitmap: RoaringBitmap::new(),
        }
    }
    pub fn add(&mut self, value: u32) {
        self.bitmap.insert(value);
    }
    pub fn remove(&mut self, value: u32) {
        self.bitmap.remove(value);
    }
    pub fn contains(&self, value: u32) -> bool {
        self.bitmap.contains(value)
    }
    pub fn is_empty(&self) -> bool {
        self.bitmap.is_empty()
    }
    pub fn len(&self) -> u64 {
        self.bitmap.len()
    }

    pub fn is_disjoint(&self, other: &BitMap) -> bool {
        self.bitmap.is_disjoint(&other.bitmap)
    }
    pub fn is_subset(&self, other: &BitMap) -> bool {
        self.bitmap.is_subset(&other.bitmap)
    }
    pub fn is_superset(&self, other: &BitMap) -> bool {
        self.bitmap.is_superset(&other.bitmap)
    }

    pub fn display(&self) -> String {
        format!("{:?}", self.bitmap)
    }
}
impl Default for BitMap {
    fn default() -> Self {
        Self::new()
    }
}
