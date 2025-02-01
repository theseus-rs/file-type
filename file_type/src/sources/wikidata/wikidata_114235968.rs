use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114235968: FileFormat = FileFormat {
    id: 114_235_968,
    puid: "wikidata/114235968",
    name: "SYBYL format",
    extensions: &["sml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
