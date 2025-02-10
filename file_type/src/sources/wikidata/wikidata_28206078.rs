use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206078: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_078,
        source_type: SourceType::Wikidata,
        name: "View ST/TT TT-Medium",
        extensions: &["PI5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
