use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1629407822: FileFormat = FileFormat {
    id: 1_629_407_822,
    source_type: SourceType::Iana,
    name: "example",
    extensions: &[],
    media_types: &["model/example"],
    internal_signatures: &[],
    related_formats: &[],
};
