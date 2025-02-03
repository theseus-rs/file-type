use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_604279: FileFormat = FileFormat {
    id: 604_279,
    source_type: SourceType::Wikidata,
    name: "Dirac",
    extensions: &["drc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
