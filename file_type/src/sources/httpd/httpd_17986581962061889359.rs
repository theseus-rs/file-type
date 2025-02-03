use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17986581962061889359: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "sfv",
    extensions: &["sfv"],
    media_types: &["text/x-sfv"],
    internal_signatures: &[],
    related_formats: &[],
};
