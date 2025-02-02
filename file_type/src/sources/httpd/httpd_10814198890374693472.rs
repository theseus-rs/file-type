use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10814198890374693472: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "djvu",
    extensions: &["djvu", "djv"],
    media_types: &["image/vnd.djvu"],
    internal_signatures: &[],
    related_formats: &[],
};
