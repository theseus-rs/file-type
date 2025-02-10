use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207008: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_008,
        source_type: SourceType::Wikidata,
        name: "Picture Publisher 4",
        extensions: &["pp4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
