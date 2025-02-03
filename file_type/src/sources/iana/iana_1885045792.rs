use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1885045792: FileFormat = FileFormat {
    id: 1_885_045_792,
    source_type: SourceType::Iana,
    name: "prs.cyn",
    extensions: &[],
    media_types: &["application/prs.cyn"],
    internal_signatures: &[],
    related_formats: &[],
};
