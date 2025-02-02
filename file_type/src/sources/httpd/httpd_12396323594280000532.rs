use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12396323594280000532: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pkcs7 certreqresp",
    extensions: &["p7r"],
    media_types: &["application/x-pkcs7-certreqresp"],
    internal_signatures: &[],
    related_formats: &[],
};
