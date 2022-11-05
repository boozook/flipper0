use super::*;
use crate::Result;
use std::io::Read;


pub(crate) fn find_read_api_table<P: AsRef<Path>>(root: P) -> Result<(Option<String>, Vec<ApiTableRow<String>>)> {
	let root = root.as_ref();
	let path = root.join(PathBuf::from("firmware/targets/f7/api_symbols.csv"));
	println!("cargo:rerun-if-changed={}", path.display());

	let file = std::fs::File::open(path)?;
	read_api_table(file)
}


/// Read csv table with api symbols
pub(crate) fn read_api_table<R: Read>(reader: R) -> Result<(Option<String>, Vec<ApiTableRow<String>>)> {
	let mut version = None;
	let mut symbols = Vec::new();

	let mut reader = csv::Reader::from_reader(reader);
	for (i, result) in reader.records().enumerate() {
		match result {
			Ok(mut record) => {
				record.trim();

				if matches!(record.get(0), Some("Version")) {
					version = record.get(2).map(ToOwned::to_owned);
				} else {
					symbols.push(record.into());
				}
			},
			Err(err) => println!("cargo:warning=API table row {i} error: {err}"),
		}
	}

	Ok((version, symbols))
}


pub(crate) fn gen_api_table_header(symbols: &[ApiTableRow<String>]) -> Result<PathBuf> {
	use Status::*;
	use ApiTableRow::*;

	let outdir = PathBuf::from(env::var("OUT_DIR")?);
	let mut result = String::new();

	// XXX: add missed import:
	result.push_str("#include <stdbool.h> \n\n");

	for h in symbols.iter().filter(|s| matches!(s, Header { status: NotRem(_), .. })) {
		let path = h.name();
		let row = format!("#include <{path}> \n");
		result.push_str(&row);
	}


	result.push_str("\n/* variables */\n\n");
	for v in symbols.iter().filter(|s| matches!(s, Variable { .. })) {
		let name = v.name();
		let ty = v.ty();
		if ty == "const Icon" {
			continue;
		}
		let _row = format!("extern {ty} {name}; \n");
		// result.push_str(&row); // not needed yet.
	}


	result.push_str("\n/* functions */\n\n");
	for f in symbols.iter().filter(|s| matches!(s, Function { .. })) {
		let name = f.name();
		let ret = f.ty();
		let args = f.args();
		let _row = format!("extern {ret} {name}({args}); \n");
		// result.push_str(&row); // not needed yet.
	}

	let path = outdir.join("flipper_bindings.h");
	std::fs::write(&path, result.as_bytes())?;

	Ok(path)
}


#[allow(dead_code)] // allow dead-code for status for future exclustions
#[derive(Debug, Clone)]
pub(crate) enum ApiTableRow<S: AsRef<str>> {
	Header { status: Status<S>, name: S },
	Variable { status: Status<S>, name: S, mut_ty: S, ty: S },
	Function { status: Status<S>, name: S, ret: S, args: S },
}


impl From<csv::StringRecord> for ApiTableRow<String> {
	fn from(row: csv::StringRecord) -> Self {
		match row.get(0).unwrap().to_lowercase().as_str() {
			"header" => {
				Self::Header { status: row.get(1).unwrap().to_owned().into(),
				               name: row.get(2).unwrap().to_owned() }
			},
			"variable" => {
				Self::Variable { status: row.get(1).unwrap().to_owned().into(),
				                 name: row.get(2).unwrap().to_owned(),
				                 mut_ty: row.get(3).unwrap().to_owned(),
				                 ty: row.get(4).unwrap().to_owned() }
			},
			"function" => {
				Self::Function { status: row.get(1).unwrap().to_owned().into(),
				                 name: row.get(2).unwrap().to_owned(),
				                 ret: row.get(3).unwrap().to_owned(),
				                 args: row.get(4).unwrap_or_default().to_owned() }
			},
			_ => {
				println!("cargo:error=Unable to parse API table row.");
				std::process::exit(1);
			},
		}
	}
}


#[allow(dead_code)]
impl<S: AsRef<str>> ApiTableRow<S> {
	pub fn name(&self) -> &str {
		match self {
			ApiTableRow::Header { name, .. } => name.as_ref(),
			ApiTableRow::Variable { name, .. } => name.as_ref(),
			ApiTableRow::Function { name, .. } => name.as_ref(),
		}
	}
	pub fn ty(&self) -> &str {
		match self {
			ApiTableRow::Header { .. } => "",
			ApiTableRow::Variable { mut_ty, .. } => mut_ty.as_ref(),
			ApiTableRow::Function { ret, .. } => ret.as_ref(),
		}
	}

	pub fn args(&self) -> &str {
		match self {
			ApiTableRow::Header { .. } => "",
			ApiTableRow::Variable { .. } => "",
			ApiTableRow::Function { args, .. } => args.as_ref(),
		}
	}

	pub fn status(&self) -> &Status<S> {
		match self {
			ApiTableRow::Header { status, .. } => status,
			ApiTableRow::Variable { status, .. } => status,
			ApiTableRow::Function { status, .. } => status,
		}
	}
}


#[derive(Debug, Clone)]
pub enum Status<S: AsRef<str> = &'static str> {
	Rem,
	NotRem(S),
}

impl<S: AsRef<str>> From<S> for Status<S> {
	fn from(value: S) -> Self {
		let s = value.as_ref();
		match s {
			"-" => Status::Rem,
			_ => Status::NotRem(value),
		}
	}
}
