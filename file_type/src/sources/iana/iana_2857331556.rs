use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2857331556: FileFormat = FileFormat {
    id: 2_857_331_556,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.wordprocessingml.template",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.template"],
    internal_signatures: &[],
    related_formats: &[],
};
