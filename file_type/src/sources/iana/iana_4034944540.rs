use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4034944540: FileType = FileType {
    file_format: &FileFormat {
        id: 4_034_944_540,
        source_type: SourceType::Iana,
        name: "FFV1",
        extensions: &[],
        media_types: &["video/FFV1"],
        signatures: &[],
        related_formats: &[],
    },
};
