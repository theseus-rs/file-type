use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113521033: FileType = FileType {
    file_format: &FileFormat {
        id: 113_521_033,
        source_type: SourceType::Wikidata,
        name: "BIM Metadata File",
        extensions: &["bim"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
