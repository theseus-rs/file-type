use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122072541: FileType = FileType {
    file_format: &FileFormat {
        id: 122_072_541,
        source_type: SourceType::Wikidata,
        name: "Rhapsody File",
        extensions: &["rhp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
