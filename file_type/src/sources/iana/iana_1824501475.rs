use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1824501475: FileFormat = FileFormat {
    id: 1_824_501_475,
    source_type: SourceType::Iana,
    name: "alto-propmapparams+json",
    extensions: &[],
    media_types: &["application/alto-propmapparams+json"],
    signatures: &[],
    related_formats: &[],
};
