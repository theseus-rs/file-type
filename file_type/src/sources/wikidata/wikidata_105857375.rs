use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857375: FileFormat = FileFormat {
    id: 105_857_375,
    puid: "wikidata/105857375",
    name: "Cura extruder definition",
    extensions: &["json"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
