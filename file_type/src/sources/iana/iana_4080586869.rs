use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4080586869: FileType = FileType {
    file_format: &FileFormat {
        id: 4_080_586_869,
        source_type: SourceType::Iana,
        name: "epub+zip",
        extensions: &[],
        media_types: &["application/epub+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
