use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2735872475: FileFormat = FileFormat {
    id: 2_735_872_475,
    source_type: SourceType::Httpd,
    name: "jam",
    extensions: &["jam"],
    media_types: &["application/vnd.jam"],
    signatures: &[],
    related_formats: &[],
};
