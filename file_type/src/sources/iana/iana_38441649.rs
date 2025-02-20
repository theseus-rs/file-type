use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_38441649: FileType = FileType {
    file_format: &FileFormat {
        id: 38_441_649,
        source_type: SourceType::Iana,
        name: "sarif+json",
        extensions: &[],
        media_types: &["application/sarif+json"],
        signatures: &[],
        related_formats: &[],
    },
};
