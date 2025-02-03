use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_212159430: FileFormat = FileFormat {
    id: 212_159_430,
    source_type: SourceType::Httpd,
    name: "mac compactpro",
    extensions: &["cpt"],
    media_types: &["application/mac-compactpro"],
    internal_signatures: &[],
    related_formats: &[],
};
