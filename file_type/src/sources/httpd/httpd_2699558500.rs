use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2699558500: FileFormat = FileFormat {
    id: 2_699_558_500,
    source_type: SourceType::Httpd,
    name: "hp hps",
    extensions: &["hps"],
    media_types: &["application/vnd.hp-hps"],
    internal_signatures: &[],
    related_formats: &[],
};
