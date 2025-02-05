use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2222656293: FileFormat = FileFormat {
    id: 2_222_656_293,
    source_type: SourceType::Iana,
    name: "application/trust-mark+jwt",
    extensions: &[],
    media_types: &["application/trust-mark+jwt"],
    signatures: &[],
    related_formats: &[],
};
