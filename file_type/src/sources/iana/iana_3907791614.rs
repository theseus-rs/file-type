use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3907791614: FileType = FileType {
    file_format: &FileFormat {
        id: 3_907_791_614,
        source_type: SourceType::Iana,
        name: "vnd.cyclonedx+xml",
        extensions: &[],
        media_types: &["application/vnd.cyclonedx+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
