use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2638993316: FileFormat = FileFormat {
    id: 2_638_993_316,
    source_type: SourceType::Iana,
    name: "vnd.hp-hpid",
    extensions: &[],
    media_types: &["application/vnd.hp-hpid"],
    internal_signatures: &[],
    related_formats: &[],
};
