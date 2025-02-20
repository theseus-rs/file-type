use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1554586014: FileType = FileType {
    file_format: &FileFormat {
        id: 1_554_586_014,
        source_type: SourceType::Iana,
        name: "token-introspection+jwt",
        extensions: &[],
        media_types: &["application/token-introspection+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
