use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3386034869: FileFormat = FileFormat {
    id: 3_386_034_869,
    source_type: SourceType::Httpd,
    name: "pkcs7 certreqresp",
    extensions: &["p7r"],
    media_types: &["application/x-pkcs7-certreqresp"],
    internal_signatures: &[],
    related_formats: &[],
};
