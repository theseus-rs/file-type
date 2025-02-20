use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206066: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_066,
        source_type: SourceType::Wikidata,
        name: "View ST/TT TT-Low",
        extensions: &["PI4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
