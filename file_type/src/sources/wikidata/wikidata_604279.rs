use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_604279: FileFormat = FileFormat {
    id: 604_279,
    source_type: SourceType::Wikidata,
    name: "Dirac",
    extensions: &["drc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
