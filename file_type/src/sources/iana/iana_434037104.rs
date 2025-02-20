use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_434037104: FileType = FileType {
    file_format: &FileFormat {
        id: 434_037_104,
        source_type: SourceType::Iana,
        name: "at+jwt",
        extensions: &[],
        media_types: &["application/at+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
