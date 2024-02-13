use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::time::SystemTime;

use aes::cipher::{KeyIvInit, StreamCipher as _};
use base64::prelude::*;
use hmac::digest::Output;
use hmac::{Hmac, Mac};
use libsignal_protocol::{
    kem, GenericSignedPreKey, IdentityKeyStore, KeyPair, KyberPreKeyRecord, PreKeyRecord, PrivateKey,
    PublicKey, SignalProtocolError, SignedPreKeyRecord,
};
use prost::Message;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::libsignal_service::errors::ServiceError;
use crate::signalservice::DeviceName;

type Aes256Ctr128BE = ctr::Ctr128BE<aes::Aes256>;

fn calculate_hmac256(mac_key: &[u8], ciphertext: &[u8]) -> Result<Output<Hmac<Sha256>>, ServiceError> {
    let mut mac = Hmac::<Sha256>::new_from_slice(mac_key).map_err(|_| ServiceError::MacError)?;
    mac.update(ciphertext);
    Ok(mac.finalize().into_bytes())
}

pub fn encrypt_device_name<R: rand::Rng + rand::CryptoRng>(
    csprng: &mut R,
    device_name: &str,
    identity_public: &PublicKey,
) -> Result<DeviceName, ServiceError> {
    let plaintext = device_name.as_bytes().to_vec();
    let ephemeral_key_pair = KeyPair::generate(csprng);

    let master_secret = ephemeral_key_pair.private_key.calculate_agreement(identity_public)?;

    let key1 = calculate_hmac256(&master_secret, b"auth")?;
    let synthetic_iv = calculate_hmac256(&key1, &plaintext)?;
    let synthetic_iv = &synthetic_iv[..16];

    let key2 = calculate_hmac256(&master_secret, b"cipher")?;
    let cipher_key = calculate_hmac256(&key2, synthetic_iv)?;

    let mut ciphertext = plaintext;

    const IV: [u8; 16] = [0; 16];
    let mut cipher = Aes256Ctr128BE::new(&cipher_key, &IV.into());
    cipher.apply_keystream(&mut ciphertext);

    let device_name = DeviceName {
        ephemeral_public: Some(ephemeral_key_pair.public_key.serialize().to_vec()),
        synthetic_iv: Some(synthetic_iv.to_vec()),
        ciphertext: Some(ciphertext),
    };

    Ok(device_name)
}

pub fn decrypt_device_name(
    private_key: &PrivateKey,
    device_name: &DeviceName,
) -> Result<String, ServiceError> {
    let DeviceName {
        ephemeral_public: Some(ephemeral_public),
        synthetic_iv: Some(synthetic_iv),
        ciphertext: Some(ciphertext),
    } = device_name
    else {
        return Err(ServiceError::InvalidDeviceName);
    };

    let synthetic_iv: [u8; 16] =
        synthetic_iv[..synthetic_iv.len().min(16)].try_into().map_err(|_| ServiceError::MacError)?;

    let ephemeral_public = PublicKey::deserialize(ephemeral_public)?;

    let master_secret = private_key.calculate_agreement(&ephemeral_public)?;
    let key2 = calculate_hmac256(&master_secret, b"cipher")?;
    let cipher_key = calculate_hmac256(&key2, &synthetic_iv)?;

    let mut plaintext = ciphertext.to_vec();
    const IV: [u8; 16] = [0; 16];
    let mut cipher = Aes256Ctr128BE::new(cipher_key.as_slice().into(), &IV.into());
    cipher.apply_keystream(&mut plaintext);

    let key1 = calculate_hmac256(&master_secret, b"auth")?;
    let our_synthetic_iv = calculate_hmac256(&key1, &plaintext)?;
    let our_synthetic_iv = &our_synthetic_iv[..16];

    if synthetic_iv != our_synthetic_iv {
        Err(ServiceError::MacError)
    } else {
        Ok(String::from_utf8_lossy(&plaintext).to_string())
    }
}

#[cfg(test)]
mod tests {
    use base64::Engine;
    use libsignal_protocol::{KeyPair, PrivateKey, PublicKey};

    use super::DeviceName;
    use crate::utils::BASE64_RELAXED;

    #[test]
    fn encrypt_device_name() -> anyhow::Result<()> {
        let input_device_name = "Nokia 3310 Millenial Edition";
        let mut csprng = rand::thread_rng();
        let identity = KeyPair::generate(&mut csprng);

        let device_name = super::encrypt_device_name(&mut csprng, input_device_name, &identity.public_key)?;

        let decrypted_device_name = super::decrypt_device_name(&identity.private_key, &device_name)?;

        assert_eq!(input_device_name, decrypted_device_name);

        Ok(())
    }

    #[test]
    fn decrypt_device_name() -> anyhow::Result<()> {
        let ephemeral_private_key =
            PrivateKey::deserialize(&BASE64_RELAXED.decode("0CgxHjwwblXjvX8sD5wZDWdYToMRf+CZSlgaUrxCGVo=")?)?;
        let ephemeral_public_key =
            PublicKey::deserialize(&BASE64_RELAXED.decode("BcZS+Lt6yAKbEpXnRX+I5wHqesuvu93Q2V+fjidwW8R6")?)?;

        let device_name = DeviceName {
            ephemeral_public: Some(ephemeral_public_key.serialize().to_vec()),
            synthetic_iv: Some(BASE64_RELAXED.decode("86gekHGmltnnZ9QARhiFcg==")?),
            ciphertext: Some(BASE64_RELAXED.decode("MtJ9/9KBWLBVAxfZJD4pLKzP4q+iodRJeCc+/A==")?),
        };

        let decrypted_device_name = super::decrypt_device_name(&ephemeral_private_key, &device_name)?;

        assert_eq!(decrypted_device_name, "Nokia 3310 Millenial Edition");

        Ok(())
    }
}
