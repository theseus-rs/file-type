use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1402999297: FileType = FileType {
    file_format: &FileFormat {
        id: 1_402_999_297,
        source_type: SourceType::Iana,
        name: "matroska",
        extensions: &[],
        media_types: &["video/matroska"],
        signatures: &[],
        related_formats: &[],
    },
};
