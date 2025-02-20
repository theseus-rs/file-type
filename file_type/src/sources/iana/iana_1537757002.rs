use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1537757002: FileType = FileType {
    file_format: &FileFormat {
        id: 1_537_757_002,
        source_type: SourceType::Iana,
        name: "EDIFACT",
        extensions: &[],
        media_types: &["application/EDIFACT"],
        signatures: &[],
        related_formats: &[],
    },
};
