type Error = Box<dyn std::error::Error>;
type Result<T = (), E = self::Error> = std::result::Result<T, E>;


fn main() -> Result {
	let fam = fam::manifest()?;
	let path = fam.save_to_out_dir()?;

	println!("Exported FAM path: {}", path.display());

	Ok(())
}
