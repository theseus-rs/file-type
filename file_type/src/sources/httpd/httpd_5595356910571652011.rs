use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5595356910571652011: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "umajin",
    extensions: &["umj"],
    media_types: &["application/vnd.umajin"],
    internal_signatures: &[],
    related_formats: &[],
};
