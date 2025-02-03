use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2638993316: FileFormat = FileFormat {
    id: 2_638_993_316,
    source_type: SourceType::Httpd,
    name: "hp hpid",
    extensions: &["hpid"],
    media_types: &["application/vnd.hp-hpid"],
    internal_signatures: &[],
    related_formats: &[],
};
