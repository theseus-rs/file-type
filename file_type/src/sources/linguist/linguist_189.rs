use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_189: FileFormat = FileFormat {
    id: 189,
    source_type: SourceType::Linguist,
    name: "Kotlin",
    extensions: &["kt", "ktm", "kts"],
    media_types: &["text/x-kotlin"],
    internal_signatures: &[],
    related_formats: &[],
};
