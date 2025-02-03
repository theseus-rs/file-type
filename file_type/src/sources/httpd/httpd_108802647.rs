use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_108802647: FileFormat = FileFormat {
    id: 108_802_647,
    source_type: SourceType::Httpd,
    name: "xhtml xml",
    extensions: &["xhtml", "xht"],
    media_types: &["application/xhtml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
