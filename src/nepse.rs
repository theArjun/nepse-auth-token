use anyhow::Result;
use tracing::{debug};
use crate::models::{AuthenticateProveResponse, ParsedTokenResult, SaltArrays};
use crate::crypto::{calculate_crypto_indices, parse_access_token_with_indices};
use crate::NepseCryptography;

const AUTHENTICATE_PROVE_URL: &str = "https://nepalstock.com/api/authenticate/prove";

pub async fn make_request(url: &str) -> Result<AuthenticateProveResponse, reqwest::Error> {
    debug!("🌐 Making request to NEPSE API");
    let response: AuthenticateProveResponse = reqwest::get(url)
        .await?
        .json::<AuthenticateProveResponse>()
        .await?;
    debug!("📨 Received response from NEPSE API");
    Ok(response)
}

pub async fn get_access_token() -> Result<ParsedTokenResult, Box<dyn std::error::Error>> {
    let response = make_request(AUTHENTICATE_PROVE_URL).await?;
    
    debug!("🔧 Initializing WASM cryptography module");
    // Create the WASM instance for cryptographic functions
    let mut nepse_cryptography = NepseCryptography::new("wasm-modules/css.wasm")?;
    
    debug!("🧂 Processing salt values: [{}, {}, {}, {}, {}]", 
           response.salt1, response.salt2, response.salt3, response.salt4, response.salt5);
    
    // Create salt arrays in different orders
    let salt_arrays = SaltArrays::new(
        response.salt1,
        response.salt2,
        response.salt3,
        response.salt4,
        response.salt5,
    );
    
    debug!("🔢 Calculating cryptographic indices");
    // Calculate cryptographic indices
    let crypto_indices = calculate_crypto_indices(&mut nepse_cryptography, &salt_arrays)?;
    
    debug!("🔍 Parsing access token with calculated indices");
    // Parse the access token using the calculated indices
    let parsed_access_token =
        parse_access_token_with_indices(&response.access_token, &crypto_indices)?;
    
    debug!("🎯 Successfully processed access token");
    Ok(ParsedTokenResult {
        access_token: parsed_access_token,
    })
} 