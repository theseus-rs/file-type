use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2811351098: FileType = FileType {
    file_format: &FileFormat {
        id: 2_811_351_098,
        source_type: SourceType::Iana,
        name: "calendar+json",
        extensions: &[],
        media_types: &["application/calendar+json"],
        signatures: &[],
        related_formats: &[],
    },
};
