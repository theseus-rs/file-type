use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1685128235: FileType = FileType {
    file_format: &FileFormat {
        id: 1_685_128_235,
        source_type: SourceType::Iana,
        name: "city+json",
        extensions: &[],
        media_types: &["application/city+json"],
        signatures: &[],
        related_formats: &[],
    },
};
