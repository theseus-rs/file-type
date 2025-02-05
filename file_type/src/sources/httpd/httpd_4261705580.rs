use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4261705580: FileFormat = FileFormat {
    id: 4_261_705_580,
    source_type: SourceType::Httpd,
    name: "stuffit",
    extensions: &["sit"],
    media_types: &["application/x-stuffit"],
    signatures: &[],
    related_formats: &[],
};
