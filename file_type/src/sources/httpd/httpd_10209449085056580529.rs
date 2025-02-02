use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10209449085056580529: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "msmediaview",
    extensions: &["mvb", "m13", "m14"],
    media_types: &["application/x-msmediaview"],
    internal_signatures: &[],
    related_formats: &[],
};
