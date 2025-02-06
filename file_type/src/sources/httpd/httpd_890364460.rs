use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_890364460: FileFormat = FileFormat {
    id: 890_364_460,
    source_type: SourceType::Httpd,
    name: "wais source",
    extensions: &["src"],
    media_types: &["application/x-wais-source"],
    signatures: &[],
    related_formats: &[],
};
