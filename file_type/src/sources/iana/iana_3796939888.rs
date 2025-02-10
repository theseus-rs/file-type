use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3796939888: FileType = FileType {
    file_format: &FileFormat {
        id: 3_796_939_888,
        source_type: SourceType::Iana,
        name: "urc-ressheet+xml",
        extensions: &[],
        media_types: &["application/urc-ressheet+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
