use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2964301109: FileFormat = FileFormat {
    id: 2_964_301_109,
    source_type: SourceType::Iana,
    name: "sieve",
    extensions: &[],
    media_types: &["application/sieve"],
    internal_signatures: &[],
    related_formats: &[],
};
