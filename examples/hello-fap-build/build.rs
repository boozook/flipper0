type Error = Box<dyn std::error::Error>;
type Result<T = (), E = self::Error> = std::result::Result<T, E>;


fn main() -> Result {
	assert!(
	        std::env::var_os(cfg::consts::env::FLIPPER_SDK_PATH_ENV).is_some(),
	        "Unable to build because don't know where Flipper SDK is."
	);

	fap::build(true)?;
	Ok(())
}
