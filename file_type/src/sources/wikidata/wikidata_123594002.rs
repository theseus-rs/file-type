use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123594002: FileType = FileType {
    file_format: &FileFormat {
        id: 123_594_002,
        source_type: SourceType::Wikidata,
        name: "Norton Change Directory Persistent Cache File",
        extensions: &["ncd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
