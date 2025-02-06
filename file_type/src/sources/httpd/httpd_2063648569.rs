use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2063648569: FileFormat = FileFormat {
    id: 2_063_648_569,
    source_type: SourceType::Httpd,
    name: "groove tool template",
    extensions: &["tpl"],
    media_types: &["application/vnd.groove-tool-template"],
    signatures: &[],
    related_formats: &[],
};
