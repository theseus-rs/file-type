use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855712: FileFormat = FileFormat {
    id: 105_855_712,
    puid: "wikidata/105855712",
    name: "LightWave 3D exported object",
    extensions: &["obj"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
