use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2238037173: FileFormat = FileFormat {
    id: 2_238_037_173,
    source_type: SourceType::Httpd,
    name: "wsdl xml",
    extensions: &["wsdl"],
    media_types: &["application/wsdl+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
