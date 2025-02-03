use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11731180303083797159: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms cab compressed",
    extensions: &["cab"],
    media_types: &["application/vnd.ms-cab-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
