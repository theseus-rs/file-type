use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125692127: FileType = FileType {
    file_format: &FileFormat {
        id: 125_692_127,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Presentation Template",
        extensions: &["otp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
