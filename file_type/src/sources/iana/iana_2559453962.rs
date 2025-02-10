use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2559453962: FileType = FileType {
    file_format: &FileFormat {
        id: 2_559_453_962,
        source_type: SourceType::Iana,
        name: "calendar",
        extensions: &[],
        media_types: &["text/calendar"],
        signatures: &[],
        related_formats: &[],
    },
};
