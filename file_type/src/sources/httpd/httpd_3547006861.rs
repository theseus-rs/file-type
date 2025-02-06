use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3547006861: FileFormat = FileFormat {
    id: 3_547_006_861,
    source_type: SourceType::Httpd,
    name: "dtbresource xml",
    extensions: &["res"],
    media_types: &["application/x-dtbresource+xml"],
    signatures: &[],
    related_formats: &[],
};
