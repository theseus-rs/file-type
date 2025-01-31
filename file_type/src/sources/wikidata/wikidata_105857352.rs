use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857352: FileFormat = FileFormat {
    id: 105_857_352,
    puid: "wikidata/105857352",
    name: "Vega visualization",
    extensions: &["json"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
