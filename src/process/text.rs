use std::{fs, io::Read};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

use crate::{get_reader, TextSignFormat};

pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

struct Blake3 {
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

// struct Ed25519Signer {
//     key: [u8; 32],
// }

// struct Ed25519Verifier {
//     key: [u8; 32],
// }

pub trait TextVerify {
    fn verify(&self, reader: impl Read, sig: &[u8]) -> anyhow::Result<bool>;
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::hash(&buf);
        Ok(hash.as_bytes() == sig)
    }
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let key = fs::read(key)?;
            let key = &key[..32];
            let key = key.try_into()?;
            let signer = Blake3 { key };
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => todo!(),
    };
    let signed = URL_SAFE_NO_PAD.encode(signed);
    println!("{}", signed);
    Ok(())
}
