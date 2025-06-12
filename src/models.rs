use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticateProveResponse {
    pub access_token: String,
    pub is_display_active: bool,
    pub popup_doc_for: String,
    pub refresh_token: String,
    pub salt: String,
    pub salt1: i32,
    pub salt2: i32,
    pub salt3: i32,
    pub salt4: i32,
    pub salt5: i32,
    pub server_time: i64,
    pub token_type: String,
}

#[derive(Debug, Serialize)]
pub struct ParsedTokenResult {
    pub access_token: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug)]
pub struct CryptoIndices {
    pub cdx: i32,
    pub rdx: i32,
    pub bdx: i32,
    pub ndx: i32,
    pub mdx: i32,
}

#[derive(Debug)]
pub struct SaltArrays {
    pub serial_order: [i32; 5],
    pub altered_order: [i32; 5],
}

impl SaltArrays {
    pub fn new(salt1: i32, salt2: i32, salt3: i32, salt4: i32, salt5: i32) -> Self {
        Self {
            serial_order: [salt1, salt2, salt3, salt4, salt5],
            altered_order: [salt1, salt2, salt4, salt3, salt5], // swap salt3 and salt4
        }
    }
} 