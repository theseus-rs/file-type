use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112944069: FileFormat = FileFormat {
    id: 112_944_069,
    source_type: SourceType::Wikidata,
    name: "GameExchange2 animation file",
    extensions: &["GAF"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
