use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3368114516: FileFormat = FileFormat {
    id: 3_368_114_516,
    source_type: SourceType::Iana,
    name: "vnd.yamaha.openscoreformat",
    extensions: &[],
    media_types: &["application/vnd.yamaha.openscoreformat"],
    signatures: &[],
    related_formats: &[],
};
