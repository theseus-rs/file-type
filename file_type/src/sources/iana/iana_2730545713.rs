use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2730545713: FileFormat = FileFormat {
    id: 2_730_545_713,
    source_type: SourceType::Iana,
    name: "merge-patch+json",
    extensions: &[],
    media_types: &["application/merge-patch+json"],
    internal_signatures: &[],
    related_formats: &[],
};
