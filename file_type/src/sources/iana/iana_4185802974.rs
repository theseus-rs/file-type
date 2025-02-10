use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4185802974: FileType = FileType {
    file_format: &FileFormat {
        id: 4_185_802_974,
        source_type: SourceType::Iana,
        name: "alto-propmap+json",
        extensions: &[],
        media_types: &["application/alto-propmap+json"],
        signatures: &[],
        related_formats: &[],
    },
};
