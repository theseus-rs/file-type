use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_820780064: FileFormat = FileFormat {
    id: 820_780_064,
    source_type: SourceType::Httpd,
    name: "kenameaapp",
    extensions: &["htke"],
    media_types: &["application/vnd.kenameaapp"],
    signatures: &[],
    related_formats: &[],
};
