use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3193316013: FileType = FileType {
    file_format: &FileFormat {
        id: 3_193_316_013,
        source_type: SourceType::Iana,
        name: "mbms-user-service-description+xml",
        extensions: &[],
        media_types: &["application/mbms-user-service-description+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
