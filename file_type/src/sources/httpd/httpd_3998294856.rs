use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3998294856: FileFormat = FileFormat {
    id: 3_998_294_856,
    source_type: SourceType::Httpd,
    name: "xaml xml",
    extensions: &["xaml"],
    media_types: &["application/xaml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
