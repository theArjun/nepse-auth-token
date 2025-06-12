use anyhow::Result;
use nepse_auth_token::NepseCryptography;
use serde::Deserialize;

const AUTHENTICATE_PROVE_URL: &str = "https://nepalstock.com/api/authenticate/prove";


#[derive(Deserialize, Debug)]
struct AuthenticateProveResponse {
    accessToken: String,
    isDisplayActive: bool,
    popupDocFor: String,
    refreshToken: String,
    salt: String,
    salt1: i32,
    salt2: i32,
    salt3: i32,
    salt4: i32,
    salt5: i32,
    serverTime: i64,
    tokenType: String,
}

#[derive(Debug)]
struct ParsedTokenResult {
    access_token: String,
}

#[derive(Debug)]
struct CryptoIndices {
    cdx: i32,
    rdx: i32,
    bdx: i32,
    ndx: i32,
    mdx: i32,
}

#[derive(Debug)]
struct SaltArrays {
    serial_order: [i32; 5],
    altered_order: [i32; 5],
}

impl SaltArrays {
    fn new(salt1: i32, salt2: i32, salt3: i32, salt4: i32, salt5: i32) -> Self {
        Self {
            serial_order: [salt1, salt2, salt3, salt4, salt5],
            altered_order: [salt1, salt2, salt4, salt3, salt5], // swap salt3 and salt4
        }
    }
}

async fn make_request(url: &str) -> Result<AuthenticateProveResponse, reqwest::Error> {
    let response: AuthenticateProveResponse = reqwest::get(url).await?.json::<AuthenticateProveResponse>().await?;
    Ok(response)
}

fn calculate_crypto_indices(
    nepse_cryptography: &mut NepseCryptography,
    salt_arrays: &SaltArrays,
) -> Result<CryptoIndices, Box<dyn std::error::Error>> {
    let serial = &salt_arrays.serial_order;
    let altered = &salt_arrays.altered_order;

    let cdx = nepse_cryptography.cdx(serial[0], serial[1], serial[2], serial[3], serial[4])?;
    let rdx = nepse_cryptography.rdx(altered[0], altered[1], altered[2], altered[3], altered[4])?;
    let bdx = nepse_cryptography.bdx(altered[0], altered[1], altered[2], altered[3], altered[4])?;
    let ndx = nepse_cryptography.ndx(altered[0], altered[1], altered[2], altered[3], altered[4])?;
    let mdx = nepse_cryptography.mdx(altered[0], altered[1], altered[2], altered[3], altered[4])?;

    Ok(CryptoIndices { cdx, rdx, bdx, ndx, mdx })
}

fn parse_access_token_with_indices(
    access_token: &str,
    indices: &CryptoIndices,
) -> Result<String, Box<dyn std::error::Error>> {
    let cdx_usize = indices.cdx as usize;
    let rdx_usize = indices.rdx as usize;
    let bdx_usize = indices.bdx as usize;
    let ndx_usize = indices.ndx as usize;
    let mdx_usize = indices.mdx as usize;

    // Ensure indices are within bounds
    let max_len = access_token.len();
    let all_indices = [cdx_usize, rdx_usize, bdx_usize, ndx_usize, mdx_usize];
    
    if all_indices.iter().any(|&idx| idx >= max_len) {
        return Err("Calculated indices are out of bounds".into());
    }

    // Sort indices to ensure proper ordering
    let mut sorted_indices = all_indices.to_vec();
    sorted_indices.sort();
    println!("Sorted indices: {:?}", sorted_indices);

    // Build the parsed token by removing characters at the calculated positions
    let mut parsed_access_token = String::new();
    let mut last_pos = 0;

    for &idx in &sorted_indices {
        if idx >= last_pos {
            parsed_access_token.push_str(&access_token[last_pos..idx]);
            last_pos = idx + 1;
        }
    }

    // Add remaining characters
    if last_pos < max_len {
        parsed_access_token.push_str(&access_token[last_pos..]);
    }

    Ok(parsed_access_token)
}

async fn get_access_token() -> Result<ParsedTokenResult, Box<dyn std::error::Error>> {
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
    let parsed_access_token = parse_access_token_with_indices(&response.accessToken, &crypto_indices)?;
    
    Ok(ParsedTokenResult {
        access_token: parsed_access_token,
    })
}

#[tokio::main]
async fn main() {
    match get_access_token().await {
        Ok(result) => {
            println!("Parsed access token: {}", result.access_token);
        }
        Err(e) => {
            eprintln!("Error getting access token: {}", e);
        }
    }
}