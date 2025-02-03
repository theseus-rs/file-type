use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7715231678766573797: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pkcs7 signature",
    extensions: &["p7s"],
    media_types: &["application/pkcs7-signature"],
    internal_signatures: &[],
    related_formats: &[],
};
