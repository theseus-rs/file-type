use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52425808: FileFormat = FileFormat {
    id: 52_425_808,
    puid: "wikidata/52425808",
    name: "Vista Pro Graphics",
    extensions: &["dem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
