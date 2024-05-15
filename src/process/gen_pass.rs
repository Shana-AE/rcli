use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*()_-=[]{}:,.<>?";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng();
    let mut password: Vec<u8> = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        if let Some(c) = UPPER.choose(&mut rng) {
            password.push(*c);
        }
    }

    if lower {
        chars.extend_from_slice(LOWER);
        if let Some(c) = LOWER.choose(&mut rng) {
            password.push(*c);
        }
    }

    if number {
        chars.extend_from_slice(NUMBER);
        if let Some(c) = NUMBER.choose(&mut rng) {
            password.push(*c);
        }
    }

    if symbol {
        chars.extend_from_slice(SYMBOL);
        if let Some(c) = SYMBOL.choose(&mut rng) {
            password.push(*c);
        }
    }

    for _ in 0..length - password.len() as u8 {
        if let Some(c) = chars.choose(&mut rng) {
            password.push(*c);
        } else {
            anyhow::bail!("Invalid character set")
        }
    }

    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;

    Ok(password)
}
