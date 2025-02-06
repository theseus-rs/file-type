use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1877580679: FileFormat = FileFormat {
    id: 1_877_580_679,
    source_type: SourceType::Iana,
    name: "GSM",
    extensions: &[],
    media_types: &["audio/GSM"],
    signatures: &[],
    related_formats: &[],
};
