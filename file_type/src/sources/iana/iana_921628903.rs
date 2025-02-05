use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_921628903: FileFormat = FileFormat {
    id: 921_628_903,
    source_type: SourceType::Iana,
    name: "vnd.ms-works",
    extensions: &[],
    media_types: &["application/vnd.ms-works"],
    signatures: &[],
    related_formats: &[],
};
