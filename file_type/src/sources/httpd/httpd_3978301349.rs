use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3978301349: FileFormat = FileFormat {
    id: 3_978_301_349,
    source_type: SourceType::Httpd,
    name: "dtbook xml",
    extensions: &["dtb"],
    media_types: &["application/x-dtbook+xml"],
    signatures: &[],
    related_formats: &[],
};
