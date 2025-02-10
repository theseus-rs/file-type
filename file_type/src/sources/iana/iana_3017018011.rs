use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3017018011: FileType = FileType {
    file_format: &FileFormat {
        id: 3_017_018_011,
        source_type: SourceType::Iana,
        name: "header-set",
        extensions: &[],
        media_types: &["multipart/header-set"],
        signatures: &[],
        related_formats: &[],
    },
};
