use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15288519035999881865: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "omdoc xml",
    extensions: &["omdoc"],
    media_types: &["application/omdoc+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
