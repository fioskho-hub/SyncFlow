use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LwwRegister<T> {
    pub value: T,
    pub timestamp: u64,
}

impl<T: Clone> LwwRegister<T> {
    pub fn new(value: T, timestamp: u64) -> Self {
        Self { value, timestamp }
    }

    pub fn merge(&mut self, other: Self) -> bool {
        if other.timestamp > self.timestamp {
            self.value = other.value;
            self.timestamp = other.timestamp;
            true
        } else {
            false 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lww_merge_newer_wins() {
        let mut reg_a = LwwRegister::new("Titre local", 100);
        let reg_b = LwwRegister::new("Titre distant", 200);

        let updated = reg_a.merge(reg_b);

        assert!(updated);
        assert_eq!(reg_a.value, "Titre distant");
        assert_eq!(reg_a.timestamp, 200);
    }

    #[test]
    fn test_lww_merge_older_ignored() {
        let mut reg_a = LwwRegister::new("Titre récent", 300);
        let reg_b = LwwRegister::new("Titre ancien", 150);

        let updated = reg_a.merge(reg_b);

        assert!(!updated);
        assert_eq!(reg_a.value, "Titre récent");
    }
}