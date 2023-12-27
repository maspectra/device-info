pub mod aes {
    use aes_gcm::{
        aead::{Aead, AeadCore, KeyInit, OsRng},
        Aes128Gcm, Error, Key, Nonce,
    };
    use std::str;

    use base64::{engine::general_purpose, Engine as _};

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Secret {
        n: Vec<u8>,
        v: Vec<u8>,
    }

    pub fn generate_aes_key(key: Option<&String>) -> Key<Aes128Gcm> {
        match key {
            Some(value) => *Key::<Aes128Gcm>::from_slice(value.as_ref()),
            None => match std::env::var("ENCRYPTION_KEY") {
                Ok(key) => *Key::<Aes128Gcm>::from_slice(key.as_ref()),
                Err(_) => Aes128Gcm::generate_key(&mut OsRng),
            },
        }
    }

    pub fn encrypt(key: &Key<Aes128Gcm>, value: &str) -> Result<String, Error> {
        let cipher = Aes128Gcm::new(key);
        let nonce = Aes128Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let encrypted = cipher.encrypt(&nonce, value.as_ref())?;

        let secret = Secret {
            n: nonce.clone().to_vec(),
            v: encrypted.clone(),
        };
        let encoded =
            general_purpose::STANDARD_NO_PAD.encode(serde_json::to_string(&secret).unwrap());

        Ok(encoded)
    }

    pub fn decrypt(key: &Key<Aes128Gcm>, encoded: &str) -> Result<String, Error> {
        let cipher = Aes128Gcm::new(key);
        let decoded_buffer = general_purpose::STANDARD_NO_PAD.decode(encoded).unwrap();
        let decoded_str = match std::str::from_utf8(&decoded_buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        let decoded_secret = serde_json::from_str::<Secret>(decoded_str).unwrap();
        let decoded_nonce = Nonce::from_slice(&decoded_secret.n);
        let buffer = cipher.decrypt(decoded_nonce, decoded_secret.v.as_ref())?;
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
    use aes_gcm::{aead::OsRng, Aes128Gcm, KeyInit};

    #[test]
    fn test_encrypt() {
        let key = Aes128Gcm::generate_key(&mut OsRng);
        let encrypted = aes::encrypt(&key, "Hello World");

        assert!(encrypted.is_ok());
    }

    #[test]
    fn test_decrypt() {
        let key = Aes128Gcm::generate_key(&mut OsRng);
        let encrypted = aes::encrypt(&key, "Hello World");
        let decrypted = aes::decrypt(&key, &encrypted.unwrap());

        assert!(decrypted.is_ok());
        assert_eq!(decrypted.unwrap(), "Hello World");
    }

    #[test]
    fn test_device_info() {
        let key = Aes128Gcm::generate_key(&mut OsRng);
        let mut builder = MainDeviceInfoBuilder::new();
        builder.add_user_name().add_os_distro().add_platform_name();

        let encrypted = aes::encrypt(&key, serde_json::to_string(&builder).unwrap().as_str());
        let decrypted = aes::decrypt(&key, &encrypted.unwrap());

        assert!(decrypted.is_ok());

        serde_json::from_str::<MainDeviceInfoBuilder>(&decrypted.unwrap()).unwrap();
    }
}
