use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4014908057: FileFormat = FileFormat {
    id: 4_014_908_057,
    source_type: SourceType::Httpd,
    name: "mng",
    extensions: &["mng"],
    media_types: &["video/x-mng"],
    internal_signatures: &[],
    related_formats: &[],
};
