use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2260535582776616: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "adobe fxp",
    extensions: &["fxp", "fxpl"],
    media_types: &["application/vnd.adobe.fxp"],
    internal_signatures: &[],
    related_formats: &[],
};
