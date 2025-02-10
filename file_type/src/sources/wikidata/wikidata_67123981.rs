use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_67123981: FileType = FileType {
    file_format: &FileFormat {
        id: 67_123_981,
        source_type: SourceType::Wikidata,
        name: "Print Artist craft file format",
        extensions: &["crf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
