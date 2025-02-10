use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207469: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_469,
        source_type: SourceType::Wikidata,
        name: "Vivid IMG",
        extensions: &["img"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
