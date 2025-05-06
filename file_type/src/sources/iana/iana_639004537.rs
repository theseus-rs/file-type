use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_639004537: FileType = FileType {
    file_format: &FileFormat {
        id: 639_004_537,
        source_type: SourceType::Iana,
        name: "vnd.cel",
        extensions: &[],
        media_types: &["application/vnd.cel"],
        signatures: &[],
        related_formats: &[],
    },
};
