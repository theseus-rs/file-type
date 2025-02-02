use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6048803440915610366: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "cdmi domain",
    extensions: &["cdmid"],
    media_types: &["application/cdmi-domain"],
    internal_signatures: &[],
    related_formats: &[],
};
