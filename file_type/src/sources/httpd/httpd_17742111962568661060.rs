use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17742111962568661060: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "xml",
    extensions: &["xml", "xsl"],
    media_types: &["application/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
