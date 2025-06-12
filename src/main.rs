use anyhow::Result;
use nepse_auth_token::NepseCryptography;
use serde::Deserialize;

const AUTHENTICATE_PROVE_URL: &str = "https://nepalstock.com/api/authenticate/prove";


// Object {
//     "accessToken": String("eyJlbmMiOiJBMTI4Q0JDLUhTMjUs2IiwiYWxnIjoiZGlyIWn0..Y7G2e73abBx4ZZlROzyzwA.9l5UUOStDYhlN154XuUa97CUkIimXPV4Omqm2EixF0feRaUETnZMD2d2fAj-mRhbUOish9Vh-5VFgRVc8rgBtD0Fvge9KbWSfOODRN3pBFSIetig5zXfhj-IdyWufqPtFQuv2xcAH0TzKDW6AwK-ErpNw.KR1H1KndJ1a9ADbFM3tQSA"),
//     "isDisplayActive": Bool(false),
//     "popupDocFor": String("jpg"),
//     "refreshToken": String("eyJlbmMiOiJBMTI4Q0JDLUhTMjU2PIiwiYWxnIjoiZGlyIn0d..rybxvsj1WlGh6BS1LUP0eQ.-lHXIqeKqZmkgWMtZkQ-rWlkKO1Ghy-_zYANNwY2hkDTsv78-wxgujFUjOaYaxPOY8sL0wQiiYhYr7tqoz34cZwPxTpOy4zlY78icQnF1WF9b0uDsRbWlfsu9dlPTlFwmHjdqbeE904P1SaZd4AvsOEdg.Mdafoo5HfLOThI_cXquoZA"),
//     "salt": String("dw/aM\"]%2o_sN6L23zM6"),
//     "salt1": Number(13827),
//     "salt2": Number(76634),
//     "salt3": Number(92567),
//     "salt4": Number(71023),
//     "salt5": Number(50874),
//     "serverTime": Number(1749747124000),
//     "tokenType": String(""),
// }

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
    salts: Vec<i32>,
}

async fn make_request(url: &str) -> Result<AuthenticateProveResponse, reqwest::Error> {
    let response: AuthenticateProveResponse = reqwest::get(url).await?.json::<AuthenticateProveResponse>().await?;
    Ok(response)
}

async fn get_access_token() -> Result<ParsedTokenResult, Box<dyn std::error::Error>> {
    let response = make_request(AUTHENTICATE_PROVE_URL).await?;
    
    // Create the WASM instance for cryptographic functions
    let mut nepse_cryptography = NepseCryptography::new("wasm-modules/css.wasm")?;
    
    // Serial order: salt1, salt2, salt3, salt4, salt5
    let serial_order = [
        response.salt1,
        response.salt2,
        response.salt3,
        response.salt4,
        response.salt5,
    ];
    
    // Altered order: salt1, salt2, salt4, salt3, salt5 (swap salt3 and salt4)
    let altered_order = [
        response.salt1,
        response.salt2,
        response.salt4,
        response.salt3,
        response.salt5,
    ];
    
    // Calculate cryptographic values
    let cdx = nepse_cryptography.cdx(serial_order[0], serial_order[1], serial_order[2], serial_order[3], serial_order[4])?;
    let rdx = nepse_cryptography.rdx(altered_order[0], altered_order[1], altered_order[3], altered_order[2], altered_order[4])?;
    let bdx = nepse_cryptography.bdx(altered_order[0], altered_order[1], altered_order[3], altered_order[2], altered_order[4])?;
    let ndx = nepse_cryptography.ndx(altered_order[0], altered_order[1], altered_order[3], altered_order[2], altered_order[4])?;
    let mdx = nepse_cryptography.mdx(altered_order[0], altered_order[1], altered_order[3], altered_order[2], altered_order[4])?;
    
    // Parse the access token using the calculated indices
    let access_token = &response.accessToken;
    
    let cdx_usize = cdx as usize;
    let rdx_usize = rdx as usize;
    let bdx_usize = bdx as usize;
    let ndx_usize = ndx as usize;
    let mdx_usize = mdx as usize;
    
    // Ensure indices are within bounds and properly ordered
    let max_len = access_token.len();
    if cdx_usize >= max_len || rdx_usize >= max_len || bdx_usize >= max_len || 
       ndx_usize >= max_len || mdx_usize >= max_len {
        return Err("Calculated indices are out of bounds".into());
    }
    
    // Sort indices to ensure proper ordering
    let mut indices = vec![cdx_usize, rdx_usize, bdx_usize, ndx_usize, mdx_usize];
    indices.sort();
    println!("Sorted indices: {:?}", indices);
    
    // Build the parsed token by removing characters at the calculated positions
    let mut parsed_access_token = String::new();
    let mut last_pos = 0;
    
    for &idx in &indices {
        if idx < max_len && idx >= last_pos {
            parsed_access_token.push_str(&access_token[last_pos..idx]);
            last_pos = idx + 1;
        }
    }
    
    // Add remaining characters
    if last_pos < max_len {
        parsed_access_token.push_str(&access_token[last_pos..]);
    }
    
    // Create salt arrays
    let salts_arrays = vec![
        response.salt1,
        response.salt2,
        response.salt3,
        response.salt4,
        response.salt5,
    ];
    
    Ok(ParsedTokenResult {
        access_token: parsed_access_token,
        salts: salts_arrays,
    })
}

#[tokio::main]
async fn main() {
    match get_access_token().await {
        Ok(result) => {
            println!("Parsed access token: {}", result.access_token);
            println!("Salts: {:?}", result.salts);
        }
        Err(e) => {
            eprintln!("Error getting access token: {}", e);
        }
    }
}