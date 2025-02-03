use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4134001168: FileFormat = FileFormat {
    id: 4_134_001_168,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.presentationml.presentation",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
    internal_signatures: &[],
    related_formats: &[],
};
