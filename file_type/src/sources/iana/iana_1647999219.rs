use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1647999219: FileType = FileType {
    file_format: &FileFormat {
        id: 1_647_999_219,
        source_type: SourceType::Iana,
        name: "RED",
        extensions: &[],
        media_types: &["text/RED"],
        signatures: &[],
        related_formats: &[],
    },
};
