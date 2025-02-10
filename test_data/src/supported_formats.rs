use file_type::sources;
use std::fs;
use std::path::PathBuf;

pub(crate) fn generate() -> anyhow::Result<()> {
    let mut file_types = sources::file_types().collect::<Vec<_>>();
    file_types.sort();

    let file_type_table = file_types
        .iter()
        .map(|file_type| {
            let source = format!("{:?}", file_type.source_type());
            let id = file_type.id();
            let name = file_type.name();
            let media_types = file_type.media_types().join(", ");
            let extensions = file_type.extensions().join(", ");
            format!("| {source} | {id} | {name} | {extensions} | {media_types} |")
        })
        .collect::<Vec<String>>();

    let file_type_table = file_type_table.join("\n");
    let file_type_table = [
        format!("# File Types ({})\n", file_types.len()),
        "| Source | Id | Name | Extensions | Media Types |".to_string(),
        "| ---- | ---- | ---- | ----------- | ---------- |".to_string(),
        file_type_table,
    ]
    .join("\n");

    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = PathBuf::from(crate_dir).join("..").join("FILETYPES.md");
    fs::write(file_name, file_type_table)?;
    Ok(())
}
