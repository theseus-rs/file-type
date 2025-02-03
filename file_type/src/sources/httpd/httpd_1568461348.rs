use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1568461348: FileFormat = FileFormat {
    id: 1_568_461_348,
    source_type: SourceType::Httpd,
    name: "data vision rdz",
    extensions: &["rdz"],
    media_types: &["application/vnd.data-vision.rdz"],
    internal_signatures: &[],
    related_formats: &[],
};
