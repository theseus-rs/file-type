use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207102: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_102,
        source_type: SourceType::Wikidata,
        name: "The Print Shop Names file",
        extensions: &["nam"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
