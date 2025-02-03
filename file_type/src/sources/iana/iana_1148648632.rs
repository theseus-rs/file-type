use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1148648632: FileFormat = FileFormat {
    id: 1_148_648_632,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.presentationml.slide+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.slide+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
