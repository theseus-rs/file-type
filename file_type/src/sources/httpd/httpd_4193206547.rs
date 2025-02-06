use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4193206547: FileFormat = FileFormat {
    id: 4_193_206_547,
    source_type: SourceType::Httpd,
    name: "hp hpgl",
    extensions: &["hpgl"],
    media_types: &["application/vnd.hp-hpgl"],
    signatures: &[],
    related_formats: &[],
};
