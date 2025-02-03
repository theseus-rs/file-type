use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_505998768: FileFormat = FileFormat {
    id: 505_998_768,
    source_type: SourceType::Iana,
    name: "cwt",
    extensions: &[],
    media_types: &["application/cwt"],
    internal_signatures: &[],
    related_formats: &[],
};
