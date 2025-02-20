use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125692002: FileType = FileType {
    file_format: &FileFormat {
        id: 125_692_002,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Graphic Template",
        extensions: &["otg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
