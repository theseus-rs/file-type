use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1783359439: FileFormat = FileFormat {
    id: 1_783_359_439,
    source_type: SourceType::Iana,
    name: "wita",
    extensions: &[],
    media_types: &["application/wita"],
    signatures: &[],
    related_formats: &[],
};
