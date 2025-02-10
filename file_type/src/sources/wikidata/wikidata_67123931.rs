use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_67123931: FileType = FileType {
    file_format: &FileFormat {
        id: 67_123_931,
        source_type: SourceType::Wikidata,
        name: "Print Artist banner file format",
        extensions: &["ban"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
