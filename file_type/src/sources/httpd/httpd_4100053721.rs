use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4100053721: FileFormat = FileFormat {
    id: 4_100_053_721,
    source_type: SourceType::Httpd,
    name: "adobe fxp",
    extensions: &["fxp", "fxpl"],
    media_types: &["application/vnd.adobe.fxp"],
    signatures: &[],
    related_formats: &[],
};
