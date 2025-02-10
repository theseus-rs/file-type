use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3328258557: FileType = FileType {
    file_format: &FileFormat {
        id: 3_328_258_557,
        source_type: SourceType::Iana,
        name: "mp4",
        extensions: &[],
        media_types: &["audio/mp4"],
        signatures: &[],
        related_formats: &[],
    },
};
