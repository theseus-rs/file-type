use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3546824522: FileFormat = FileFormat {
    id: 3_546_824_522,
    source_type: SourceType::Httpd,
    name: "groove injector",
    extensions: &["grv"],
    media_types: &["application/vnd.groove-injector"],
    internal_signatures: &[],
    related_formats: &[],
};
