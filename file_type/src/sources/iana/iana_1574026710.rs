use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1574026710: FileFormat = FileFormat {
    id: 1_574_026_710,
    source_type: SourceType::Iana,
    name: "ipp",
    extensions: &[],
    media_types: &["application/ipp"],
    internal_signatures: &[],
    related_formats: &[],
};
