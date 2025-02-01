use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857532: FileFormat = FileFormat {
    id: 105_857_532,
    puid: "wikidata/105857532",
    name: "Indigo Renderer Material",
    extensions: &["igm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
