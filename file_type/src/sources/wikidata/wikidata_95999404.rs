use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_95999404: FileType = FileType {
    file_format: &FileFormat {
        id: 95_999_404,
        source_type: SourceType::Wikidata,
        name: "Graph6",
        extensions: &["g6"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
