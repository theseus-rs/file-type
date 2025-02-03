use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3372644123074246335: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "groove injector",
    extensions: &["grv"],
    media_types: &["application/vnd.groove-injector"],
    internal_signatures: &[],
    related_formats: &[],
};
