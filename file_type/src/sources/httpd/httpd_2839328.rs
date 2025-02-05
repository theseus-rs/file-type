use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2839328: FileFormat = FileFormat {
    id: 2_839_328,
    source_type: SourceType::Httpd,
    name: "cu seeme",
    extensions: &["cu"],
    media_types: &["application/cu-seeme"],
    signatures: &[],
    related_formats: &[],
};
