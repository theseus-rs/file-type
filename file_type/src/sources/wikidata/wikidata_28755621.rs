use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28755621: FileType = FileType {
    file_format: &FileFormat {
        id: 28_755_621,
        source_type: SourceType::Wikidata,
        name: "Exact Yearbook ID file",
        extensions: &["id"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
