use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_25503902: FileType = FileType {
    file_format: &FileFormat {
        id: 25_503_902,
        source_type: SourceType::Iana,
        name: "related",
        extensions: &[],
        media_types: &["multipart/related"],
        signatures: &[],
        related_formats: &[],
    },
};
