use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1462109934: FileFormat = FileFormat {
    id: 1_462_109_934,
    source_type: SourceType::Httpd,
    name: "mbox",
    extensions: &["mbox"],
    media_types: &["application/mbox"],
    internal_signatures: &[],
    related_formats: &[],
};
