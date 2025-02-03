use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12506654617699971632: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "wais source",
    extensions: &["src"],
    media_types: &["application/x-wais-source"],
    internal_signatures: &[],
    related_formats: &[],
};
