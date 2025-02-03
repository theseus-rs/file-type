use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2466606380: FileFormat = FileFormat {
    id: 2_466_606_380,
    source_type: SourceType::Iana,
    name: "vnd.hp-HPGL",
    extensions: &[],
    media_types: &["application/vnd.hp-HPGL"],
    internal_signatures: &[],
    related_formats: &[],
};
