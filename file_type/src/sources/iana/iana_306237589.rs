use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_306237589: FileType = FileType {
    file_format: &FileFormat {
        id: 306_237_589,
        source_type: SourceType::Iana,
        name: "mbms-associated-procedure-description+xml",
        extensions: &[],
        media_types: &["application/mbms-associated-procedure-description+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
