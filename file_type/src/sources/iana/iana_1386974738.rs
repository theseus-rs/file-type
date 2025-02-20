use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1386974738: FileType = FileType {
    file_format: &FileFormat {
        id: 1_386_974_738,
        source_type: SourceType::Iana,
        name: "calendar+xml",
        extensions: &[],
        media_types: &["application/calendar+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
