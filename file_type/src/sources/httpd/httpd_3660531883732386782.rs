use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3660531883732386782: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "amazon ebook",
    extensions: &["azw"],
    media_types: &["application/vnd.amazon.ebook"],
    internal_signatures: &[],
    related_formats: &[],
};
