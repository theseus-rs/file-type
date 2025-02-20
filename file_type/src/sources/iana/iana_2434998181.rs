use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2434998181: FileType = FileType {
    file_format: &FileFormat {
        id: 2_434_998_181,
        source_type: SourceType::Iana,
        name: "vnd.sealed.3df",
        extensions: &[],
        media_types: &["application/vnd.sealed.3df"],
        signatures: &[],
        related_formats: &[],
    },
};
