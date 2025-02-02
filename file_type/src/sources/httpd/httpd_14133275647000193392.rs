use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14133275647000193392: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "oasis opendocument text web",
    extensions: &["oth"],
    media_types: &["application/vnd.oasis.opendocument.text-web"],
    internal_signatures: &[],
    related_formats: &[],
};
