use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1329015423: FileFormat = FileFormat {
    id: 1_329_015_423,
    source_type: SourceType::Httpd,
    name: "ipfix",
    extensions: &["ipfix"],
    media_types: &["application/ipfix"],
    signatures: &[],
    related_formats: &[],
};
