use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1173754933: FileFormat = FileFormat {
    id: 1_173_754_933,
    source_type: SourceType::Httpd,
    name: "adobe xfdf",
    extensions: &["xfdf"],
    media_types: &["application/vnd.adobe.xfdf"],
    signatures: &[],
    related_formats: &[],
};
