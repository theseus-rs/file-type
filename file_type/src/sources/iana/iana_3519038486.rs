use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3519038486: FileType = FileType {
    file_format: &FileFormat {
        id: 3_519_038_486,
        source_type: SourceType::Iana,
        name: "csrattrs",
        extensions: &[],
        media_types: &["application/csrattrs"],
        signatures: &[],
        related_formats: &[],
    },
};
