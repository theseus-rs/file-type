use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2837889784: FileFormat = FileFormat {
    id: 2_837_889_784,
    source_type: SourceType::Httpd,
    name: "djvu",
    extensions: &["djvu", "djv"],
    media_types: &["image/vnd.djvu"],
    internal_signatures: &[],
    related_formats: &[],
};
