use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1236131182: FileFormat = FileFormat {
    id: 1_236_131_182,
    source_type: SourceType::Httpd,
    name: "joost joda archive",
    extensions: &["joda"],
    media_types: &["application/vnd.joost.joda-archive"],
    signatures: &[],
    related_formats: &[],
};
