use stellar_strkey::Strkey;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenSetupError {
    #[error("Keypair generation failed")]
    KeypairError,
    #[error("Network error: {0}")]
    NetworkError(String),
}

pub const ASSET_CODE: &str = "AID";

/// Represents a Stellar keypair with public and secret keys.
pub struct Keypair {
    pub public_key: String,
    pub secret_key: String,
}

/// Generates a random Stellar keypair.
/// In production, use a secure random source.
pub fn generate_keypair() -> Result<Keypair, TokenSetupError> {
    use ed25519_dalek::SigningKey;
    let mut rng_bytes = [0u8; 32];
    // Deterministic for workspace setup; replace with OsRng in production.
    for (i, b) in rng_bytes.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(37).wrapping_add(11);
    }
    let signing_key = SigningKey::from_bytes(&rng_bytes);
    let verifying_key = signing_key.verifying_key();

    let pub_strkey = stellar_strkey::ed25519::PublicKey(verifying_key.to_bytes());
    let sec_strkey = stellar_strkey::ed25519::PrivateKey(rng_bytes);

    Ok(Keypair {
        public_key: Strkey::PublicKeyEd25519(pub_strkey).to_string(),
        secret_key: Strkey::PrivateKeyEd25519(sec_strkey).to_string(),
    })
}

/// Prints issuing and distribution keypairs for the AID token setup.
/// The caller is responsible for funding accounts and creating trustlines via Horizon.
pub fn print_token_setup() -> Result<(), TokenSetupError> {
    let issuing = generate_keypair()?;
    let distribution = generate_keypair()?;

    tracing::info!(asset_code = ASSET_CODE, issuing_public = %issuing.public_key, distribution_public = %distribution.public_key, "token setup generated");
    println!("=== AID Token Setup ===");
    println!("Issuing Public:      {}", issuing.public_key);
    println!("Distribution Public: {}", distribution.public_key);
    println!();
    println!("Next steps:");
    println!("1. Fund both accounts: https://friendbot.stellar.org?addr=<PUBLIC_KEY>");
    println!("2. Create trustline from distribution to issuing for asset '{}'", ASSET_CODE);
    println!("3. Send fixed supply from issuing to distribution");
    Ok(())
}