use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1585678573: FileFormat = FileFormat {
    id: 1_585_678_573,
    source_type: SourceType::Iana,
    name: "moss-keys",
    extensions: &[],
    media_types: &["application/moss-keys"],
    internal_signatures: &[],
    related_formats: &[],
};
