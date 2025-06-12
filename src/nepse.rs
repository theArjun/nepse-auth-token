use anyhow::Result;
use crate::models::{AuthenticateProveResponse, ParsedTokenResult, SaltArrays};
use crate::crypto::{calculate_crypto_indices, parse_access_token_with_indices};
use crate::NepseCryptography;

const AUTHENTICATE_PROVE_URL: &str = "https://nepalstock.com/api/authenticate/prove";

pub async fn make_request(url: &str) -> Result<AuthenticateProveResponse, reqwest::Error> {
    let response: AuthenticateProveResponse = reqwest::get(url)
        .await?
        .json::<AuthenticateProveResponse>()
        .await?;
    Ok(response)
}

pub async fn get_access_token() -> Result<ParsedTokenResult, Box<dyn std::error::Error>> {
    let response = make_request(AUTHENTICATE_PROVE_URL).await?;
    
    // Create the WASM instance for cryptographic functions
    let mut nepse_cryptography = NepseCryptography::new("wasm-modules/css.wasm")?;
    
    // Create salt arrays in different orders
    let salt_arrays = SaltArrays::new(
        response.salt1,
        response.salt2,
        response.salt3,
        response.salt4,
        response.salt5,
    );
    
    // Calculate cryptographic indices
    let crypto_indices = calculate_crypto_indices(&mut nepse_cryptography, &salt_arrays)?;
    
    // Parse the access token using the calculated indices
    let parsed_access_token =
        parse_access_token_with_indices(&response.access_token, &crypto_indices)?;
    
    Ok(ParsedTokenResult {
        access_token: parsed_access_token,
    })
} 