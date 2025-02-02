use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_208976687: FileFormat = FileFormat {
    id: 208_976_687,
    source_type: SourceType::Linguist,
    name: "Sieve",
    extensions: &["sieve"],
    media_types: &["application/sieve"],
    internal_signatures: &[],
    related_formats: &[],
};
