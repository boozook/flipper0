pub mod local_sdk;
pub mod remote_sdk;


fn doc_sdk_metadata_row(rev: Option<impl AsRef<str>>, tags: &[impl AsRef<str>], api: Option<impl AsRef<str>>) -> String {
	let mut parts = Vec::new();

	if !tags.is_empty() {
		parts.push(format!(
			"git {plural}: {tags}",
			plural = if tags.len() > 1 { "tags" } else { "tag" },
			tags = tags.into_iter().map(|s| s.as_ref()).collect::<Vec<_>>().join(", ")
		));
	}

	if let Some(rev) = rev {
		let git_word = if tags.is_empty() { "git " } else { "" };
		parts.push(format!("{git_word}revision: {}", rev.as_ref()));
	}

	if let Some(version) = api {
		parts.push(format!("API version: __{}__", version.as_ref()));
	}

	format!(" Flipper Zero SDK {}.", parts.join(", "))
}
