use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10350728140267115011: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "hp hps",
    extensions: &["hps"],
    media_types: &["application/vnd.hp-hps"],
    internal_signatures: &[],
    related_formats: &[],
};
