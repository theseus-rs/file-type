use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
