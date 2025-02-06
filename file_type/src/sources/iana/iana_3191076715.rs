use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3191076715: FileFormat = FileFormat {
    id: 3_191_076_715,
    source_type: SourceType::Iana,
    name: "example",
    extensions: &[],
    media_types: &["message/example"],
    signatures: &[],
    related_formats: &[],
};
