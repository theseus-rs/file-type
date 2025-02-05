use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2321520595: FileFormat = FileFormat {
    id: 2_321_520_595,
    source_type: SourceType::Iana,
    name: "example",
    extensions: &[],
    media_types: &["multipart/example"],
    signatures: &[],
    related_formats: &[],
};
