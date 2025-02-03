use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2166160279: FileFormat = FileFormat {
    id: 2_166_160_279,
    source_type: SourceType::Iana,
    name: "vnd.cybank",
    extensions: &[],
    media_types: &["application/vnd.cybank"],
    internal_signatures: &[],
    related_formats: &[],
};
