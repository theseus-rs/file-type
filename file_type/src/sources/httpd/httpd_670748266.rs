use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_670748266: FileFormat = FileFormat {
    id: 670_748_266,
    source_type: SourceType::Httpd,
    name: "ttf",
    extensions: &["ttf"],
    media_types: &["font/ttf"],
    signatures: &[],
    related_formats: &[],
};
