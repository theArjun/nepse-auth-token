use wasmtime::*;
use anyhow::Result;

pub struct NepseCryptography {
    store: Store<()>,
    cdx_func: TypedFunc<(i32, i32, i32, i32, i32), i32>,
    rdx_func: TypedFunc<(i32, i32, i32, i32, i32), i32>,
    bdx_func: TypedFunc<(i32, i32, i32, i32, i32), i32>,
    ndx_func: TypedFunc<(i32, i32, i32, i32, i32), i32>,
    mdx_func: TypedFunc<(i32, i32, i32, i32, i32), i32>,
}

impl NepseCryptography {
    pub fn new(wasm_file_path: &str) -> Result<Self> {
        // Create the WebAssembly engine
        let engine = Engine::default();

        // Load the WASM file
        let module = Module::from_file(&engine, wasm_file_path)?;

        // Create a store
        let mut store = Store::new(&engine, ());

        // Create a linker for handling imports
        let mut linker = Linker::new(&engine);

        // Define the wasm-bindgen required import
        linker.func_wrap("wbg", "__wbindgen_init_externref_table", || {})?;

        // Instantiate the module using the linker
        let instance = linker.instantiate(&mut store, &module)?;

        // Get the exported functions
        let cdx_func = instance.get_typed_func::<(i32, i32, i32, i32, i32), i32>(&mut store, "cdx")?;
        let rdx_func = instance.get_typed_func::<(i32, i32, i32, i32, i32), i32>(&mut store, "rdx")?;
        let bdx_func = instance.get_typed_func::<(i32, i32, i32, i32, i32), i32>(&mut store, "bdx")?;
        let ndx_func = instance.get_typed_func::<(i32, i32, i32, i32, i32), i32>(&mut store, "ndx")?;
        let mdx_func = instance.get_typed_func::<(i32, i32, i32, i32, i32), i32>(&mut store, "ndx")?;

        Ok(NepseCryptography {
            store,
            cdx_func,
            rdx_func,
            bdx_func,
            ndx_func,
            mdx_func,
        })
    }

    pub fn cdx(&mut self, a: i32, b: i32, c: i32, d: i32, e: i32) -> Result<i32> {
        self.cdx_func.call(&mut self.store, (a, b, c, d, e))
    }

    pub fn rdx(&mut self, a: i32, b: i32, c: i32, d: i32, e: i32) -> Result<i32> {
        self.rdx_func.call(&mut self.store, (a, b, c, d, e))
    }

    pub fn bdx(&mut self, a: i32, b: i32, c: i32, d: i32, e: i32) -> Result<i32> {
        self.bdx_func.call(&mut self.store, (a, b, c, d, e))
    }

    pub fn ndx(&mut self, a: i32, b: i32, c: i32, d: i32, e: i32) -> Result<i32> {
        self.ndx_func.call(&mut self.store, (a, b, c, d, e))
    }

    pub fn mdx(&mut self, a: i32, b: i32, c: i32, d: i32, e: i32) -> Result<i32> {
        self.mdx_func.call(&mut self.store, (a, b, c, d, e))
    }
} 