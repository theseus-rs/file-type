use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_22: FileType = FileType {
    file_format: &FileFormat {
        id: 22,
        source_type: SourceType::Linguist,
        name: "AsciiDoc",
        extensions: &["adoc", "asc", "asciidoc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
