use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3057474112: FileType = FileType {
    file_format: &FileFormat {
        id: 3_057_474_112,
        source_type: SourceType::Iana,
        name: "smpte291",
        extensions: &[],
        media_types: &["video/smpte291"],
        signatures: &[],
        related_formats: &[],
    },
};
