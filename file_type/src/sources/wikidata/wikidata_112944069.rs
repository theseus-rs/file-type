use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112944069: FileFormat = FileFormat {
    id: 112_944_069,
    source_type: SourceType::Wikidata,
    name: "GameExchange2 animation file",
    extensions: &["GAF"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
