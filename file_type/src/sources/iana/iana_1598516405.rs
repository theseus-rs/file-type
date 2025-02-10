use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1598516405: FileType = FileType {
    file_format: &FileFormat {
        id: 1_598_516_405,
        source_type: SourceType::Iana,
        name: "xop+xml",
        extensions: &[],
        media_types: &["application/xop+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
