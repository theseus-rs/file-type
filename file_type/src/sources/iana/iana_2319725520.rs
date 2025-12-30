use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2319725520: FileType = FileType {
    file_format: &FileFormat {
        id: 2_319_725_520,
        source_type: SourceType::Iana,
        name: "3gpp-mbs-user-service-descriptions+json",
        extensions: &[],
        media_types: &["application/3gpp-mbs-user-service-descriptions+json"],
        signatures: &[],
        related_formats: &[],
    },
};
