use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3328258557: FileFormat = FileFormat {
    id: 3_328_258_557,
    source_type: SourceType::Httpd,
    name: "mp4",
    extensions: &["m4a", "mp4a"],
    media_types: &["audio/mp4"],
    internal_signatures: &[],
    related_formats: &[],
};
