use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123420503: FileType = FileType {
    file_format: &FileFormat {
        id: 123_420_503,
        source_type: SourceType::Wikidata,
        name: "DropBox file",
        extensions: &["dbox"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
