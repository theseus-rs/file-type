use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130479004: FileFormat = FileFormat {
    id: 130_479_004,
    puid: "wikidata/130479004",
    name: "Pointless source code file",
    extensions: &["ptls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
