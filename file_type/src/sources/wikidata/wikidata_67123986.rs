use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_67123986: FileType = FileType {
    file_format: &FileFormat {
        id: 67_123_986,
        source_type: SourceType::Wikidata,
        name: "Print Artist envelope file format",
        extensions: &["env"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
