use wasmtime::*;
use anyhow::Result;

fn main() -> Result<()> {
    // Create the WebAssembly engine
    let engine = Engine::default();

    // Load the WASM file
    let module = Module::from_file(&engine, "wasm-modules/css.wasm")?;

    // Create a store
    let mut store = Store::new(&engine, ());

    // Create a linker for handling imports
    let mut linker = Linker::new(&engine);

    // Define the wasm-bindgen required import
    linker.func_wrap("wbg", "__wbindgen_init_externref_table", || {})?;

    // Instantiate the module using the linker
    let instance = linker.instantiate(&mut store, &module)?;

    // Get the exported functions
    let cdx = instance.get_typed_func::<(i32, i32, i32, i32, i32), i32>(&mut store, "cdx")?;
    let rdx = instance.get_typed_func::<(i32, i32, i32, i32, i32), i32>(&mut store, "rdx")?;
    let bdx = instance.get_typed_func::<(i32, i32, i32, i32, i32), i32>(&mut store, "bdx")?;
    let ndx = instance.get_typed_func::<(i32, i32, i32, i32, i32), i32>(&mut store, "ndx")?;
    let mdx = instance.get_typed_func::<(i32, i32, i32, i32, i32), i32>(&mut store, "ndx")?;

    // Call the functions and print results
    println!("CDX: {}", cdx.call(&mut store, (10, 5, 3, 2, 1))?); // 26
    println!("RDX: {}", rdx.call(&mut store, (10, 5, 3, 2, 1))?); // 36
    println!("BDX: {}", bdx.call(&mut store, (10, 5, 3, 2, 1))?); // 64
    println!("NDX: {}", ndx.call(&mut store, (10, 5, 3, 2, 1))?); // 92
    println!("MDX: {}", mdx.call(&mut store, (10, 5, 3, 2, 1))?); // 92

    Ok(())
}