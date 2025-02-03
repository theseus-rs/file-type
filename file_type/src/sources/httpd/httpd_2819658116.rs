use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2819658116: FileFormat = FileFormat {
    id: 2_819_658_116,
    source_type: SourceType::Httpd,
    name: "amazon ebook",
    extensions: &["azw"],
    media_types: &["application/vnd.amazon.ebook"],
    internal_signatures: &[],
    related_formats: &[],
};
