pub struct Encrypter {
    key: Vec<u8>,
}

impl Encrypter {
    pub fn new(key: impl Into<Vec<u8>>) -> Self {
        Self { key: key.into() }
    }

    pub fn encrypt_decrypt(&self, data: &[u8]) -> Vec<u8> {
        if self.key.is_empty() {
            return data.to_vec();
        }
        data.iter()
            .enumerate()
            .map(|(i, &byte)| byte ^ self.key[i % self.key.len()])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption_roundtrip() {
        let key = b"secret_key";
        let encrypter = Encrypter::new(key);
        let original_data = b"SyncFlow Data Payload";

        let encrypted = encrypter.encrypt_decrypt(original_data);
        assert_ne!(original_data, encrypted.as_slice());

        let decrypted = encrypter.encrypt_decrypt(&encrypted);
        assert_eq!(original_data, decrypted.as_slice());
    }
}