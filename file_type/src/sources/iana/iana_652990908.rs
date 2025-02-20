use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_652990908: FileType = FileType {
    file_format: &FileFormat {
        id: 652_990_908,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.presentationml.tags+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.presentationml.tags+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
