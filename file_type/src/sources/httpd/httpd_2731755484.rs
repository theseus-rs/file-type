use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2731755484: FileFormat = FileFormat {
    id: 2_731_755_484,
    source_type: SourceType::Httpd,
    name: "csh",
    extensions: &["csh"],
    media_types: &["application/x-csh"],
    internal_signatures: &[],
    related_formats: &[],
};
