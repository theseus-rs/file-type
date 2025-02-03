use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_105059212: FileFormat = FileFormat {
    id: 105_059_212,
    source_type: SourceType::Iana,
    name: "ATXML",
    extensions: &[],
    media_types: &["application/ATXML"],
    internal_signatures: &[],
    related_formats: &[],
};
