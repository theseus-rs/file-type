use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2403512592: FileFormat = FileFormat {
    id: 2_403_512_592,
    source_type: SourceType::Iana,
    name: "automationml-aml+xml",
    extensions: &[],
    media_types: &["application/automationml-aml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
