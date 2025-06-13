use serde::{Deserialize, Serialize};
use wasmtime::{Engine, Linker, Module, Store, TypedFunc};

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

pub struct NepseCryptography {
    store: Store<()>,
    cdx_func: TypedFunc<(i32, i32, i32, i32, i32), i32>,
    rdx_func: TypedFunc<(i32, i32, i32, i32, i32), i32>,
    bdx_func: TypedFunc<(i32, i32, i32, i32, i32), i32>,
    ndx_func: TypedFunc<(i32, i32, i32, i32, i32), i32>,
    mdx_func: TypedFunc<(i32, i32, i32, i32, i32), i32>,
}

impl NepseCryptography {
    pub fn new(wasm_file_path: &str) -> anyhow::Result<Self> {
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
        let cdx_func =
            instance.get_typed_func::<(i32, i32, i32, i32, i32), i32>(&mut store, "cdx")?;
        let rdx_func =
            instance.get_typed_func::<(i32, i32, i32, i32, i32), i32>(&mut store, "rdx")?;
        let bdx_func =
            instance.get_typed_func::<(i32, i32, i32, i32, i32), i32>(&mut store, "bdx")?;
        let ndx_func =
            instance.get_typed_func::<(i32, i32, i32, i32, i32), i32>(&mut store, "ndx")?;
        let mdx_func =
            instance.get_typed_func::<(i32, i32, i32, i32, i32), i32>(&mut store, "ndx")?;

        Ok(NepseCryptography {
            store,
            cdx_func,
            rdx_func,
            bdx_func,
            ndx_func,
            mdx_func,
        })
    }

    pub fn cdx(&mut self, a: i32, b: i32, c: i32, d: i32, e: i32) -> anyhow::Result<i32> {
        self.cdx_func.call(&mut self.store, (a, b, c, d, e))
    }

    pub fn rdx(&mut self, a: i32, b: i32, c: i32, d: i32, e: i32) -> anyhow::Result<i32> {
        self.rdx_func.call(&mut self.store, (a, b, c, d, e))
    }

    pub fn bdx(&mut self, a: i32, b: i32, c: i32, d: i32, e: i32) -> anyhow::Result<i32> {
        self.bdx_func.call(&mut self.store, (a, b, c, d, e))
    }

    pub fn ndx(&mut self, a: i32, b: i32, c: i32, d: i32, e: i32) -> anyhow::Result<i32> {
        self.ndx_func.call(&mut self.store, (a, b, c, d, e))
    }

    pub fn mdx(&mut self, a: i32, b: i32, c: i32, d: i32, e: i32) -> anyhow::Result<i32> {
        self.mdx_func.call(&mut self.store, (a, b, c, d, e))
    }
}
