use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3083309523: FileFormat = FileFormat {
    id: 3_083_309_523,
    source_type: SourceType::Httpd,
    name: "businessobjects",
    extensions: &["rep"],
    media_types: &["application/vnd.businessobjects"],
    internal_signatures: &[],
    related_formats: &[],
};
