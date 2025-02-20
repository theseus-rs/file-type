use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_76178463: FileType = FileType {
    file_format: &FileFormat {
        id: 76_178_463,
        source_type: SourceType::Iana,
        name: "vnd.amadeus+json",
        extensions: &[],
        media_types: &["application/vnd.amadeus+json"],
        signatures: &[],
        related_formats: &[],
    },
};
