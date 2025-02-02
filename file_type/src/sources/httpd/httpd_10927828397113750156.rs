use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10927828397113750156: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pkcs7 certificates",
    extensions: &["p7b", "spc"],
    media_types: &["application/x-pkcs7-certificates"],
    internal_signatures: &[],
    related_formats: &[],
};
