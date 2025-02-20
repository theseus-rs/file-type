use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123385339: FileType = FileType {
    file_format: &FileFormat {
        id: 123_385_339,
        source_type: SourceType::Wikidata,
        name: "Object library file",
        extensions: &["obl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
