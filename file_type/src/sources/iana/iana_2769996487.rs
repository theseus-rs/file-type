use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2769996487: FileType = FileType {
    file_format: &FileFormat {
        id: 2_769_996_487,
        source_type: SourceType::Iana,
        name: "vnd.apple.mpegurl",
        extensions: &[],
        media_types: &["application/vnd.apple.mpegurl"],
        signatures: &[],
        related_formats: &[],
    },
};
