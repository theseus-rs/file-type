use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3280513078: FileType = FileType {
    file_format: &FileFormat {
        id: 3_280_513_078,
        source_type: SourceType::Iana,
        name: "turtle",
        extensions: &[],
        media_types: &["text/turtle"],
        signatures: &[],
        related_formats: &[],
    },
};
