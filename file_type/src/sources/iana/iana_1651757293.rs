use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1651757293: FileFormat = FileFormat {
    id: 1_651_757_293,
    source_type: SourceType::Iana,
    name: "vnd.ms-word.document.macroEnabled.12",
    extensions: &[],
    media_types: &["application/vnd.ms-word.document.macroEnabled.12"],
    signatures: &[],
    related_formats: &[],
};
