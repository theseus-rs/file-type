use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_875380544: FileFormat = FileFormat {
    id: 875_380_544,
    source_type: SourceType::Iana,
    name: "DII",
    extensions: &[],
    media_types: &["application/DII"],
    signatures: &[],
    related_formats: &[],
};
