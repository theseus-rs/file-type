use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858289: FileFormat = FileFormat {
    id: 105_858_289,
    puid: "wikidata/105858289",
    name: "E:D Shipyard ship loadout",
    extensions: &["txt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
