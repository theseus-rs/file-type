use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3224209250085143021: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "rpki roa",
    extensions: &["roa"],
    media_types: &["application/rpki-roa"],
    internal_signatures: &[],
    related_formats: &[],
};
