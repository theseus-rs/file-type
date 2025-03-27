use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_11240154: FileType = FileType {
    file_format: &FileFormat {
        id: 11_240_154,
        source_type: SourceType::Wikidata,
        name: "XLD4 Q4",
        extensions: &["q4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
