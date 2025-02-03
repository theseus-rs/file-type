use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17488117191040683567: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "msterminal",
    extensions: &["trm"],
    media_types: &["application/x-msterminal"],
    internal_signatures: &[],
    related_formats: &[],
};
