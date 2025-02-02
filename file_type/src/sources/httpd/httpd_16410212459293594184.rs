use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16410212459293594184: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "rar compressed",
    extensions: &["rar"],
    media_types: &["application/x-rar-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
