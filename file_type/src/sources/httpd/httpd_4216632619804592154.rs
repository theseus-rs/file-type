use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4216632619804592154: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pkcs7 mime",
    extensions: &["p7m", "p7c"],
    media_types: &["application/pkcs7-mime"],
    internal_signatures: &[],
    related_formats: &[],
};
