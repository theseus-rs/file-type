use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112944069: FileFormat = FileFormat {
    id: 112_944_069,
    puid: "wikidata/112944069",
    name: "GameExchange2 animation file",
    extensions: &["GAF"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
