use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1574748413: FileType = FileType {
    file_format: &FileFormat {
        id: 1_574_748_413,
        source_type: SourceType::Iana,
        name: "vnd.api+json",
        extensions: &[],
        media_types: &["application/vnd.api+json"],
        signatures: &[],
        related_formats: &[],
    },
};
