use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3328258557: FileType = FileType {
    file_format: &FileFormat {
        id: 3_328_258_557,
        source_type: SourceType::Httpd,
        name: "mp4",
        extensions: &["m4a", "mp4a"],
        media_types: &["audio/mp4"],
        signatures: &[],
        related_formats: &[],
    },
};
