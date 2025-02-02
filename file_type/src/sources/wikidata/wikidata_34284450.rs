use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34284450: FileFormat = FileFormat {
    id: 34_284_450,
    source_type: SourceType::Wikidata,
    name: "Pawn script",
    extensions: &["p"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
