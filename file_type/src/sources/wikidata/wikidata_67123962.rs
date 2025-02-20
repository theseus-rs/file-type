use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_67123962: FileType = FileType {
    file_format: &FileFormat {
        id: 67_123_962,
        source_type: SourceType::Wikidata,
        name: "Print Artist calendar file format",
        extensions: &["cal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
