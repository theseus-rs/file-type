use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1016595786: FileFormat = FileFormat {
    id: 1_016_595_786,
    source_type: SourceType::Iana,
    name: "inkml+xml",
    extensions: &[],
    media_types: &["application/inkml+xml"],
    signatures: &[],
    related_formats: &[],
};
