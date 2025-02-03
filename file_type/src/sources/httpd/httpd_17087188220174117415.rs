use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17087188220174117415: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mp4",
    extensions: &["mp4", "mp4v", "mpg4"],
    media_types: &["video/mp4"],
    internal_signatures: &[],
    related_formats: &[],
};
