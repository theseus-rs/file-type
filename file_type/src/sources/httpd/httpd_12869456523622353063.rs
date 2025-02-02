use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12869456523622353063: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms photo",
    extensions: &["wdp"],
    media_types: &["image/vnd.ms-photo"],
    internal_signatures: &[],
    related_formats: &[],
};
