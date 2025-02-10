use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3514429168: FileType = FileType {
    file_format: &FileFormat {
        id: 3_514_429_168,
        source_type: SourceType::Iana,
        name: "vnd.street-stream",
        extensions: &[],
        media_types: &["application/vnd.street-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
