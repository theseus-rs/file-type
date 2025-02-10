use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_152753445: FileType = FileType {
    file_format: &FileFormat {
        id: 152_753_445,
        source_type: SourceType::Iana,
        name: "SGML",
        extensions: &[],
        media_types: &["text/SGML"],
        signatures: &[],
        related_formats: &[],
    },
};
