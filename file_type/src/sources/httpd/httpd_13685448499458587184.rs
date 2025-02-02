use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13685448499458587184: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "h263",
    extensions: &["h263"],
    media_types: &["video/h263"],
    internal_signatures: &[],
    related_formats: &[],
};
