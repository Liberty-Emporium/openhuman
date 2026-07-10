//! JSON-RPC / CLI controller surface for encryption-focused helpers.

use crate::alexander_ai_solutions::config::Config;
use crate::rpc::RpcOutcome;

pub async fn encrypt_secret(
    config: &Config,
    plaintext: &str,
) -> Result<RpcOutcome<String>, String> {
    crate::alexander_ai_solutions::credentials::rpc::encrypt_secret(config, plaintext).await
}

pub async fn decrypt_secret(
    config: &Config,
    ciphertext: &str,
) -> Result<RpcOutcome<String>, String> {
    crate::alexander_ai_solutions::credentials::rpc::decrypt_secret(config, ciphertext).await
}
