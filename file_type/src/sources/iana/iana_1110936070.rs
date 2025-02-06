use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1110936070: FileFormat = FileFormat {
    id: 1_110_936_070,
    source_type: SourceType::Iana,
    name: "vnd.ms-word.template.macroEnabled.12",
    extensions: &[],
    media_types: &["application/vnd.ms-word.template.macroEnabled.12"],
    signatures: &[],
    related_formats: &[],
};
