use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206404: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_404,
        source_type: SourceType::Wikidata,
        name: "JEDMICS C4",
        extensions: &["c4", "ct4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
