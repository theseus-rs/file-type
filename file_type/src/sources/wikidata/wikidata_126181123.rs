use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126181123: FileFormat = FileFormat {
    id: 126_181_123,
    source_type: SourceType::Wikidata,
    name: "Graphisoft Archicad Project 6-9",
    extensions: &["pla", "pln"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
