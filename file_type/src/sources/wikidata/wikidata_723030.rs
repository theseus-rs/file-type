use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_723030: FileType = FileType {
    file_format: &FileFormat {
        id: 723_030,
        source_type: SourceType::Wikidata,
        name: "AsciiDoc",
        extensions: &["adoc", "asciidoc", "txt"],
        media_types: &["text/asciidoc"],
        signatures: &[],
        related_formats: &[],
    },
};
