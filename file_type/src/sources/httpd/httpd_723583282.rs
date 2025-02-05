use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_723583282: FileFormat = FileFormat {
    id: 723_583_282,
    source_type: SourceType::Httpd,
    name: "cdmi object",
    extensions: &["cdmio"],
    media_types: &["application/cdmi-object"],
    signatures: &[],
    related_formats: &[],
};
