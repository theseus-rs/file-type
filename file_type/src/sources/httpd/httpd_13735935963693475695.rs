use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13735935963693475695: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "tei xml",
    extensions: &["tei", "teicorpus"],
    media_types: &["application/tei+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
