use std::ops::Deref;

use super::{hidden_service_secret_key::Data, Error, HiddenServiceSecretKey, Result};

pub struct ExpandedSecretKey(ed25519_dalek::hazmat::ExpandedSecretKey);

impl ExpandedSecretKey {
    #[must_use]
    pub fn generate() -> Self {
        todo!()
        // let mut csprng = rand_07::rngs::OsRng {};
        // let secret_key = ed25519_dalek::SecretKey::generate(&mut csprng);
        // Self(ed25519_dalek::hazmat::ExpandedSecretKey::from(&secret_key))
    }

    pub fn to_bytes(&self) -> [u8; 64] {
        let scalar = self.scalar.to_bytes();
        let hash_prefix = self.hash_prefix;

        let mut bytes: [u8; 64] = [0; 64];

        let (left, right) = bytes.split_at_mut(32);
        left.copy_from_slice(&scalar);
        right.copy_from_slice(&hash_prefix);

        bytes
    }
}

impl Deref for ExpandedSecretKey {
    type Target = ed25519_dalek::hazmat::ExpandedSecretKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&HiddenServiceSecretKey> for ExpandedSecretKey {
    type Error = Error;

    fn try_from(value: &HiddenServiceSecretKey) -> Result<Self, Self::Error> {
        match &**value {
            Data::Ed25519V1Type0(data) => Ok(Self(
                ed25519_dalek::hazmat::ExpandedSecretKey::from_bytes(data),
            )),
        }
    }
}
