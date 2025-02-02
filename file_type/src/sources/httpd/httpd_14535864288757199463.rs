use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14535864288757199463: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "quicktime",
    extensions: &["qt", "mov"],
    media_types: &["video/quicktime"],
    internal_signatures: &[],
    related_formats: &[],
};
