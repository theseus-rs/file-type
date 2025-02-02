use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_22: FileFormat = FileFormat {
    id: 22,
    source_type: SourceType::Linguist,
    name: "AsciiDoc",
    extensions: &["adoc", "asc", "asciidoc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
