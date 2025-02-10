use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3131680240: FileType = FileType {
    file_format: &FileFormat {
        id: 3_131_680_240,
        source_type: SourceType::Iana,
        name: "H263",
        extensions: &[],
        media_types: &["video/H263"],
        signatures: &[],
        related_formats: &[],
    },
};
