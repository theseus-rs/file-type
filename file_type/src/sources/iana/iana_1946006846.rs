use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1946006846: FileType = FileType {
    file_format: &FileFormat {
        id: 1_946_006_846,
        source_type: SourceType::Iana,
        name: "alto-cdnifilter+json",
        extensions: &[],
        media_types: &["application/alto-cdnifilter+json"],
        signatures: &[],
        related_formats: &[],
    },
};
