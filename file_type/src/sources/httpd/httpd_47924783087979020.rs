use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_47924783087979020: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "reginfo xml",
    extensions: &["rif"],
    media_types: &["application/reginfo+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
