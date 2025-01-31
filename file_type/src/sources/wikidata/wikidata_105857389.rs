use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857389: FileFormat = FileFormat {
    id: 105_857_389,
    puid: "wikidata/105857389",
    name: "JSON Entity Model",
    extensions: &["jem"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
