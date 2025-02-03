use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2805301633: FileFormat = FileFormat {
    id: 2_805_301_633,
    source_type: SourceType::Httpd,
    name: "curl mcurl",
    extensions: &["mcurl"],
    media_types: &["text/vnd.curl.mcurl"],
    internal_signatures: &[],
    related_formats: &[],
};
