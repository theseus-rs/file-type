use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3558484330179102172: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "seemail",
    extensions: &["see"],
    media_types: &["application/vnd.seemail"],
    internal_signatures: &[],
    related_formats: &[],
};
