use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1089237792: FileFormat = FileFormat {
    id: 1_089_237_792,
    source_type: SourceType::Httpd,
    name: "postscript",
    extensions: &["ai", "eps", "ps"],
    media_types: &["application/postscript"],
    signatures: &[],
    related_formats: &[],
};
