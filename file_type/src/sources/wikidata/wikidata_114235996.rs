use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114235996: FileFormat = FileFormat {
    id: 114_235_996,
    puid: "wikidata/114235996",
    name: "SYBYL2 format",
    extensions: &["ml2", "sm2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
