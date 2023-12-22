pub mod aes {
    use aes_gcm::{
        aead::{Aead, AeadCore, KeyInit, OsRng},
        Aes256Gcm, Error, Key, Nonce,
    };

    use base64::{engine::general_purpose, Engine as _};

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Secret {
        nonce: Vec<u8>,
        cipher_text: Vec<u8>,
    }

    pub fn generate_key() -> Key<Aes256Gcm> {
        match std::env::var("ENCRYPTION_KEY") {
            Ok(key) => *Key::<Aes256Gcm>::from_slice(key.as_ref()),
            Err(_) => Aes256Gcm::generate_key(OsRng),
        }
    }

    pub fn encrypt(key: &Key<Aes256Gcm>, value: &str) -> Result<String, Error> {
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let cipher_text = cipher.encrypt(&nonce, value.as_ref())?;
        let secret = Secret {
            nonce: nonce.clone().to_vec(),
            cipher_text: cipher_text.clone(),
        };
        let encoded =
            general_purpose::STANDARD_NO_PAD.encode(serde_json::to_string(&secret).unwrap());

        Ok(encoded)
    }

    pub fn decrypt(key: &Key<Aes256Gcm>, encoded: &str) -> Result<String, Error> {
        let cipher = Aes256Gcm::new(key);
        let decoded_buffer = general_purpose::STANDARD_NO_PAD.decode(encoded).unwrap();
        let decoded_str = match std::str::from_utf8(&decoded_buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        let decoded_secret = serde_json::from_str::<Secret>(decoded_str).unwrap();
        let decoded_nonce = Nonce::from_slice(&decoded_secret.nonce);
        let buffer = cipher.decrypt(decoded_nonce, decoded_secret.cipher_text.as_ref())?;
        let s = match std::str::from_utf8(&buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        Ok(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{
        builder::{IMainBuilder, MainDeviceInfoBuilder},
        crypto::aes,
    };
    use aes_gcm::{aead::OsRng, Aes256Gcm, KeyInit};

    #[test]
    fn test_encrypt() {
        let key = Aes256Gcm::generate_key(OsRng);
        let encrypted = aes::encrypt(&key, "Hello World");

        assert_eq!(encrypted.is_ok(), true);
    }

    #[test]
    fn test_decrypt() {
        let key = Aes256Gcm::generate_key(OsRng);
        let encrypted = aes::encrypt(&key, "Hello World");
        let decrypted = aes::decrypt(&key, &encrypted.unwrap());

        assert_eq!(decrypted.is_ok(), true);
        assert_eq!(decrypted.unwrap(), "Hello World");
    }

    #[test]
    fn test_device_info() {
        let key = Aes256Gcm::generate_key(OsRng);
        let mut builder = MainDeviceInfoBuilder::new();
        builder.add_user_name().add_os_distro().add_platform_name();

        let encrypted = aes::encrypt(&key, builder.to_string().as_str());
        let decrypted = aes::decrypt(&key, &encrypted.unwrap());

        assert_eq!(decrypted.is_ok(), true);

        serde_json::from_str::<MainDeviceInfoBuilder>(&decrypted.unwrap()).unwrap();
    }
}
