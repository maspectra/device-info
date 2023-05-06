use crate::MainDeviceInfoBuilder;

use hex;
use hmac::{Hmac, Mac};
use md5::Md5;
use sha1::Sha1;
use sha2::Sha256;

const DEFAULT_ENCRYPTION_KEY_PREFIX: &str = "key";

pub enum EncryptionAlgorithm {
    MD5,
    SHA1,
    SHA256,
}

type HmacMd5 = Hmac<Md5>;
type HmacSha1 = Hmac<Sha1>;
type HmacSha256 = Hmac<Sha256>;

pub fn encrypt(builder: MainDeviceInfoBuilder, algorithm: EncryptionAlgorithm) -> String {
    match algorithm {
        EncryptionAlgorithm::MD5 => {
            let mut mac = HmacMd5::new_from_slice(
                std::env::var("ENCRYPTION_KEY_PREFIX")
                    .unwrap_or(DEFAULT_ENCRYPTION_KEY_PREFIX.to_string())
                    .as_bytes(),
            )
            .unwrap();
            mac.update(builder.to_string().as_bytes());

            hex::encode(mac.finalize().into_bytes())
        }
        EncryptionAlgorithm::SHA1 => {
            let mut mac = HmacSha1::new_from_slice(
                std::env::var("ENCRYPTION_KEY_PREFIX")
                    .unwrap_or(DEFAULT_ENCRYPTION_KEY_PREFIX.to_string())
                    .as_bytes(),
            )
            .unwrap();
            mac.update(builder.to_string().as_bytes());

            hex::encode(mac.finalize().into_bytes())
        }
        EncryptionAlgorithm::SHA256 => {
            let mut mac = HmacSha256::new_from_slice(
                std::env::var("ENCRYPTION_KEY_PREFIX")
                    .unwrap_or(DEFAULT_ENCRYPTION_KEY_PREFIX.to_string())
                    .as_bytes(),
            )
            .unwrap();
            mac.update(builder.to_string().as_bytes());

            hex::encode(mac.finalize().into_bytes())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let builder = MainDeviceInfoBuilder::new();
        let encrypted = encrypt(builder, EncryptionAlgorithm::MD5);
        println!("{}", encrypted);
    }
}
