use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2170054828: FileType = FileType {
    file_format: &FileFormat {
        id: 2_170_054_828,
        source_type: SourceType::Iana,
        name: "vnd.mason+json",
        extensions: &[],
        media_types: &["application/vnd.mason+json"],
        signatures: &[],
        related_formats: &[],
    },
};
