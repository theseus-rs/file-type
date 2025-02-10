use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1213052401: FileType = FileType {
    file_format: &FileFormat {
        id: 1_213_052_401,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.presentationml.comments+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.presentationml.comments+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
