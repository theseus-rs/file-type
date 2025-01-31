use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857365: FileFormat = FileFormat {
    id: 105_857_365,
    puid: "wikidata/105857365",
    name: "MAME plugin config",
    extensions: &["json"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
