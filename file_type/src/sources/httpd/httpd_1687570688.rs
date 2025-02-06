use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1687570688: FileFormat = FileFormat {
    id: 1_687_570_688,
    source_type: SourceType::Httpd,
    name: "mp4",
    extensions: &["mp4", "mp4v", "mpg4"],
    media_types: &["video/mp4"],
    signatures: &[],
    related_formats: &[],
};
