use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_510862015: FileFormat = FileFormat {
    id: 510_862_015,
    source_type: SourceType::Iana,
    name: "step+xml",
    extensions: &[],
    media_types: &["model/step+xml"],
    signatures: &[],
    related_formats: &[],
};
