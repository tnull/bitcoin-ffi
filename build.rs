fn main() {
	uniffi::generate_scaffolding("src/bitcoin_ffi.udl").unwrap();
}
