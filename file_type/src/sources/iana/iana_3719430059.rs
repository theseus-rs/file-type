use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3719430059: FileType = FileType {
    file_format: &FileFormat {
        id: 3_719_430_059,
        source_type: SourceType::Iana,
        name: "resolve-response+jwt",
        extensions: &[],
        media_types: &["application/resolve-response+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
