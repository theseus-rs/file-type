use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2070717028: FileFormat = FileFormat {
    id: 2_070_717_028,
    source_type: SourceType::Httpd,
    name: "javascript",
    extensions: &["js", "mjs"],
    media_types: &["text/javascript"],
    signatures: &[],
    related_formats: &[],
};
