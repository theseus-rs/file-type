use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16814499624578705614: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "x509 ca cert",
    extensions: &["der", "crt"],
    media_types: &["application/x-x509-ca-cert"],
    internal_signatures: &[],
    related_formats: &[],
};
