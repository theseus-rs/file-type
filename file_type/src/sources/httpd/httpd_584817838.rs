use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_584817838: FileFormat = FileFormat {
    id: 584_817_838,
    source_type: SourceType::Httpd,
    name: "docbook xml",
    extensions: &["dbk"],
    media_types: &["application/docbook+xml"],
    signatures: &[],
    related_formats: &[],
};
