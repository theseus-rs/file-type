use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111317643: FileType = FileType {
    file_format: &FileFormat {
        id: 111_317_643,
        source_type: SourceType::Wikidata,
        name: "Miles Sound System compressed DLS",
        extensions: &["mls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
