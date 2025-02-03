use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1452186027: FileFormat = FileFormat {
    id: 1_452_186_027,
    source_type: SourceType::Iana,
    name: "dashdelta",
    extensions: &[],
    media_types: &["application/dashdelta"],
    internal_signatures: &[],
    related_formats: &[],
};
