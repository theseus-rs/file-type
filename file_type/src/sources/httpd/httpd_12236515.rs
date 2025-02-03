use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12236515: FileFormat = FileFormat {
    id: 12_236_515,
    source_type: SourceType::Httpd,
    name: "pict",
    extensions: &["pic", "pct"],
    media_types: &["image/x-pict"],
    internal_signatures: &[],
    related_formats: &[],
};
