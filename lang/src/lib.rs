pub mod ast;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod functions;

pub mod utils;
pub mod builtins;
pub mod libraries;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(not(target_arch = "wasm32"))]
pub mod cli;

#[cfg(target_arch = "wasm32")]
pub struct WasmWriter;

#[cfg(target_arch = "wasm32")]
impl std::io::Write for WasmWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unsafe extern "C" {
            fn _modu_print(ptr: *const u8, len: usize);
        }
        
        unsafe {
            _modu_print(buf.as_ptr(), buf.len());
        }
        
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}