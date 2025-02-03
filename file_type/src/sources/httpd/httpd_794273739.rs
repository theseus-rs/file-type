use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_794273739: FileFormat = FileFormat {
    id: 794_273_739,
    source_type: SourceType::Httpd,
    name: "curl scurl",
    extensions: &["scurl"],
    media_types: &["text/vnd.curl.scurl"],
    internal_signatures: &[],
    related_formats: &[],
};
