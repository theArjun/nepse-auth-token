use crate::models::{CryptoIndices, SaltArrays};
use crate::NepseCryptography;

pub fn calculate_crypto_indices(
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

    Ok(CryptoIndices {
        cdx,
        rdx,
        bdx,
        ndx,
        mdx,
    })
}

pub fn parse_access_token_with_indices(
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