use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16022223428233642216: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "msword",
    extensions: &["doc", "dot"],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[],
};
