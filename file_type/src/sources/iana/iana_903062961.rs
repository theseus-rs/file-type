use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_903062961: FileType = FileType {
    file_format: &FileFormat {
        id: 903_062_961,
        source_type: SourceType::Iana,
        name: "vnd.oracle.resource+json",
        extensions: &[],
        media_types: &["application/vnd.oracle.resource+json"],
        signatures: &[],
        related_formats: &[],
    },
};
