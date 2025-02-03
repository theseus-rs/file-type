use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2397439993: FileFormat = FileFormat {
    id: 2_397_439_993,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
