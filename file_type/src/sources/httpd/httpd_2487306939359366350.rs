use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2487306939359366350: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "wsdl xml",
    extensions: &["wsdl"],
    media_types: &["application/wsdl+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
