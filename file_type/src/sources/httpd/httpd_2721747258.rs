use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2721747258: FileFormat = FileFormat {
    id: 2_721_747_258,
    source_type: SourceType::Httpd,
    name: "collection",
    extensions: &["ttc"],
    media_types: &["font/collection"],
    signatures: &[],
    related_formats: &[],
};
