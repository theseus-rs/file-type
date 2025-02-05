use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2190611199: FileFormat = FileFormat {
    id: 2_190_611_199,
    source_type: SourceType::Httpd,
    name: "intu qbo",
    extensions: &["qbo"],
    media_types: &["application/vnd.intu.qbo"],
    signatures: &[],
    related_formats: &[],
};
