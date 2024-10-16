#[cfg(feature = "std")]
fn main() {
	substrate_wasm_builder::WasmBuilder::new()
		.with_current_project()
		.export_heap_base()
		.import_memory()
		.build()
}

/// The wasm builder is deactivated when compiling
/// this crate for wasm to speed up the compilation.
///#[cfg_attr(not(feature = "std"), no_std)]
///fn main() {}

