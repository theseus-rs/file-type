use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131454123: FileType = FileType {
    file_format: &FileFormat {
        id: 131_454_123,
        source_type: SourceType::Wikidata,
        name: "Zig file format",
        extensions: &["zig"],
        media_types: &["text/zig"],
        signatures: &[],
        related_formats: &[],
    },
};
