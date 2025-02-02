use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2176582756351900283: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "flv",
    extensions: &["flv"],
    media_types: &["video/x-flv"],
    internal_signatures: &[],
    related_formats: &[],
};
