use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2650455745: FileType = FileType {
    file_format: &FileFormat {
        id: 2_650_455_745,
        source_type: SourceType::Iana,
        name: "captive+json",
        extensions: &[],
        media_types: &["application/captive+json"],
        signatures: &[],
        related_formats: &[],
    },
};
