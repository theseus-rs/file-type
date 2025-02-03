use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2948043618: FileFormat = FileFormat {
    id: 2_948_043_618,
    source_type: SourceType::Iana,
    name: "fastsoap",
    extensions: &[],
    media_types: &["application/fastsoap"],
    internal_signatures: &[],
    related_formats: &[],
};
