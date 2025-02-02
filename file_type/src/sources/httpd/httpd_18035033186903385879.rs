use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_18035033186903385879: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "turtle",
    extensions: &["ttl"],
    media_types: &["text/turtle"],
    internal_signatures: &[],
    related_formats: &[],
};
