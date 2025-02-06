use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_520871466: FileFormat = FileFormat {
    id: 520_871_466,
    source_type: SourceType::Httpd,
    name: "pkix cert",
    extensions: &["cer"],
    media_types: &["application/pkix-cert"],
    signatures: &[],
    related_formats: &[],
};
