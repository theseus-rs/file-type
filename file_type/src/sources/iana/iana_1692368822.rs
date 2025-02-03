use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1692368822: FileFormat = FileFormat {
    id: 1_692_368_822,
    source_type: SourceType::Iana,
    name: "vnd.genomatix.tuxedo",
    extensions: &[],
    media_types: &["application/vnd.genomatix.tuxedo"],
    internal_signatures: &[],
    related_formats: &[],
};
