use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7452515137701384940: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "xcap diff xml",
    extensions: &["xdf"],
    media_types: &["application/xcap-diff+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
