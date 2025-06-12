use anyhow::Result;
use nepse_auth_token::NepseCryptography;

fn main() -> Result<()> {
    // Create the WASM instance
    let mut nepse_cryptography = NepseCryptography::new("wasm-modules/css.wasm")?;

    // Call the functions and print results
    println!("CDX: {}", nepse_cryptography.cdx(10, 5, 3, 2, 1)?); // 26
    println!("RDX: {}", nepse_cryptography.rdx(10, 5, 3, 2, 1)?); // 36
    println!("BDX: {}", nepse_cryptography.bdx(10, 5, 3, 2, 1)?); // 64
    println!("NDX: {}", nepse_cryptography.ndx(10, 5, 3, 2, 1)?); // 92
    println!("MDX: {}", nepse_cryptography.mdx(10, 5, 3, 2, 1)?); // 92

    Ok(())
}