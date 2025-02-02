use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9088264634842123326: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "sema",
    extensions: &["sema"],
    media_types: &["application/vnd.sema"],
    internal_signatures: &[],
    related_formats: &[],
};
