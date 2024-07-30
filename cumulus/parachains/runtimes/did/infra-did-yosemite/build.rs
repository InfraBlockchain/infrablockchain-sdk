#[cfg(all(not(feature = "metadata-hash"), feature = "std"))]
fn main() {
	substrate_wasm_builder::WasmBuilder::build_using_defaults();
}

#[cfg(not(feature = "std"))]
fn main() {}
