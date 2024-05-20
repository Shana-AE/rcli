use std::{collections::HashMap, fs, io::Read, path::Path};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::{get_reader, TextSignFormat};

use super::gen_pass;

pub trait TextSign {
    /// Sign the data from the reader and return the signature
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

pub trait TextVerify {
    /// Verify the signature against the data from the reader
    fn verify(&self, reader: impl Read, sig: &[u8]) -> anyhow::Result<bool>;
}

pub trait TextKeyGenerator {
    fn generate() -> anyhow::Result<HashMap<&'static str, Vec<u8>>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        // TODO: improve perf by reading in chunks
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes() == sig)
    }
}

impl TextKeyGenerator for Blake3 {
    fn generate() -> anyhow::Result<HashMap<&'static str, Vec<u8>>> {
        let key = gen_pass::process_genpass(32, true, true, true, true)?;
        let key = key.as_bytes().to_vec();
        let mut map = HashMap::new();
        map.insert("blake3.txt", key);
        Ok(map)
    }
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
    fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        Ok(Self::new(key))
    }
    fn load(key: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(key)?;
        Self::try_new(&key)
    }
}

struct Ed25519Signer {
    key: SigningKey,
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf).to_bytes().to_vec();
        Ok(sig)
    }
}

impl Ed25519Signer {
    fn new(key: SigningKey) -> Self {
        Self { key }
    }
    fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        Ok(Self::new(key))
    }

    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl TextKeyGenerator for Ed25519Signer {
    fn generate() -> anyhow::Result<HashMap<&'static str, Vec<u8>>> {
        let mut csrng = OsRng;
        let sk = SigningKey::generate(&mut csrng);
        let pk = sk.verifying_key().as_bytes().to_vec();
        let sk = sk.as_bytes().to_vec();
        let mut map = HashMap::new();
        println!("sk: {:?}, pk: {:?}", sk, pk);
        map.insert("ed25519.sk", sk);
        map.insert("ed25519.pk", pk);
        Ok(map)
    }
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

impl Ed25519Verifier {
    fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        Ok(Self::new(key))
    }
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(&path)?;
        Self::try_new(&key)
    }
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    let signed = URL_SAFE_NO_PAD.encode(signed);
    println!("{}", signed);
    Ok(())
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    sig: &str,
    format: TextSignFormat,
) -> anyhow::Result<()> {
    let reader = get_reader(input)?;

    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(reader, &sig)?
        }
    };
    println!("verify result {}", verified);
    Ok(())
}

pub fn process_text_generate(
    format: TextSignFormat,
) -> anyhow::Result<HashMap<&'static str, Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() {
        let obj = Blake3::load("fixtures/blake3.txt").unwrap();
        let data = b"hello world!";
        let sig = obj.sign(&mut &data[..]).unwrap();
        assert!(obj.verify(&data[..], &sig).unwrap())
    }

    #[test]
    fn text_ed25519_sign_verify() {
        let sk = Ed25519Signer::load("fixtures/ed25519.sk").unwrap();
        let pk = Ed25519Verifier::load("fixtures/ed25519.pk").unwrap();

        let data = b"hello world!";
        let sig = sk.sign(&mut &data[..]).unwrap();
        assert!(pk.verify(&data[..], &sig).unwrap());
    }
}
