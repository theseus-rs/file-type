use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1096262999: FileFormat = FileFormat {
    id: 1_096_262_999,
    source_type: SourceType::Iana,
    name: "avcs",
    extensions: &[],
    media_types: &["image/avcs"],
    internal_signatures: &[],
    related_formats: &[],
};
