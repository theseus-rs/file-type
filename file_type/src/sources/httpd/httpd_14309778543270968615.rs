use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14309778543270968615: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pict",
    extensions: &["pic", "pct"],
    media_types: &["image/x-pict"],
    internal_signatures: &[],
    related_formats: &[],
};
