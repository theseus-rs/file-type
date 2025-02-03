use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_391104858: FileFormat = FileFormat {
    id: 391_104_858,
    source_type: SourceType::Httpd,
    name: "reginfo xml",
    extensions: &["rif"],
    media_types: &["application/reginfo+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
