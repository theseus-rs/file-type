use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1091729195: FileFormat = FileFormat {
    id: 1_091_729_195,
    source_type: SourceType::Iana,
    name: "partial",
    extensions: &[],
    media_types: &["message/partial"],
    internal_signatures: &[],
    related_formats: &[],
};
