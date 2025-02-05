use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_85710420: FileFormat = FileFormat {
    id: 85_710_420,
    source_type: SourceType::Httpd,
    name: "pkcs12",
    extensions: &["p12", "pfx"],
    media_types: &["application/x-pkcs12"],
    signatures: &[],
    related_formats: &[],
};
