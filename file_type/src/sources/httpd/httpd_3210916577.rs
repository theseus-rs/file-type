use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3210916577: FileFormat = FileFormat {
    id: 3_210_916_577,
    source_type: SourceType::Httpd,
    name: "curl dcurl",
    extensions: &["dcurl"],
    media_types: &["text/vnd.curl.dcurl"],
    signatures: &[],
    related_formats: &[],
};
