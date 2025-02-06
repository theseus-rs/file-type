use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_38062218: FileFormat = FileFormat {
    id: 38_062_218,
    source_type: SourceType::Iana,
    name: "mrb-consumer+xml",
    extensions: &[],
    media_types: &["application/mrb-consumer+xml"],
    signatures: &[],
    related_formats: &[],
};
