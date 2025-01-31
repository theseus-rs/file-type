use crate::format::FileFormat;

pub(crate) const LINGUIST_189: FileFormat = FileFormat {
    id: 189,
    puid: "linguist/189",
    name: "Kotlin",
    extensions: &["kt", "ktm", "kts"],
    media_types: &["text/x-kotlin"],
    internal_signatures: &[],
    related_formats: &[],
};
