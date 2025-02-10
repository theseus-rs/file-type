use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1930812415: FileType = FileType {
    file_format: &FileFormat {
        id: 1_930_812_415,
        source_type: SourceType::Iana,
        name: "simple-message-summary",
        extensions: &[],
        media_types: &["application/simple-message-summary"],
        signatures: &[],
        related_formats: &[],
    },
};
