use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_93376454: FileType = FileType {
    file_format: &FileFormat {
        id: 93_376_454,
        source_type: SourceType::Iana,
        name: "private-token-request",
        extensions: &[],
        media_types: &["application/private-token-request"],
        signatures: &[],
        related_formats: &[],
    },
};
