use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28919035: FileType = FileType {
    file_format: &FileFormat {
        id: 28_919_035,
        source_type: SourceType::Wikidata,
        name: "Type-1 DV AVI",
        extensions: &["avi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
