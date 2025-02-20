use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
