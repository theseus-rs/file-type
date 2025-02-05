use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2166384575: FileFormat = FileFormat {
    id: 2_166_384_575,
    source_type: SourceType::Iana,
    name: "xml",
    extensions: &[],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
