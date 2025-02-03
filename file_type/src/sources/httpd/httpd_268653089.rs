use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_268653089: FileFormat = FileFormat {
    id: 268_653_089,
    source_type: SourceType::Httpd,
    name: "oasis opendocument text web",
    extensions: &["oth"],
    media_types: &["application/vnd.oasis.opendocument.text-web"],
    internal_signatures: &[],
    related_formats: &[],
};
