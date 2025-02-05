use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_766797499: FileFormat = FileFormat {
    id: 766_797_499,
    source_type: SourceType::Httpd,
    name: "ufdl",
    extensions: &["ufd", "ufdl"],
    media_types: &["application/vnd.ufdl"],
    signatures: &[],
    related_formats: &[],
};
