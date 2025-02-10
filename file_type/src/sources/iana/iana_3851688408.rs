use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3851688408: FileType = FileType {
    file_format: &FileFormat {
        id: 3_851_688_408,
        source_type: SourceType::Iana,
        name: "x400-bp",
        extensions: &[],
        media_types: &["application/x400-bp"],
        signatures: &[],
        related_formats: &[],
    },
};
