use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3483784792: FileType = FileType {
    file_format: &FileFormat {
        id: 3_483_784_792,
        source_type: SourceType::Iana,
        name: "conference-info+xml",
        extensions: &[],
        media_types: &["application/conference-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
