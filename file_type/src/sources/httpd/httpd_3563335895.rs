use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3563335895: FileFormat = FileFormat {
    id: 3_563_335_895,
    source_type: SourceType::Httpd,
    name: "rpki roa",
    extensions: &["roa"],
    media_types: &["application/rpki-roa"],
    internal_signatures: &[],
    related_formats: &[],
};
