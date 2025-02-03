use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127604816: FileFormat = FileFormat {
    id: 127_604_816,
    source_type: SourceType::Wikidata,
    name: "AMPL model file",
    extensions: &["mod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
