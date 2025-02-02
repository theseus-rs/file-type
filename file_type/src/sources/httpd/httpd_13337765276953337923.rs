use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13337765276953337923: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mng",
    extensions: &["mng"],
    media_types: &["video/x-mng"],
    internal_signatures: &[],
    related_formats: &[],
};
