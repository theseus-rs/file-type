use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2321142532: FileFormat = FileFormat {
    id: 2_321_142_532,
    source_type: SourceType::Iana,
    name: "vnd.dreamfactory",
    extensions: &[],
    media_types: &["application/vnd.dreamfactory"],
    signatures: &[],
    related_formats: &[],
};
