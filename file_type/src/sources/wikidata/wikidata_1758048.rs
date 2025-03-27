use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1758048: FileType = FileType {
    file_format: &FileFormat {
        id: 1_758_048,
        source_type: SourceType::Wikidata,
        name: "Security Assertion Markup Language",
        extensions: &[],
        media_types: &[
            "application/samlassertion+xml",
            "application/samlmetadata+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
