use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_316812874: FileFormat = FileFormat {
    id: 316_812_874,
    source_type: SourceType::Iana,
    name: "multilingual",
    extensions: &[],
    media_types: &["multipart/multilingual"],
    signatures: &[],
    related_formats: &[],
};
