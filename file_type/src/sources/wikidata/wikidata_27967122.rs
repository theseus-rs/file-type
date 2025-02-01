use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967122: FileFormat = FileFormat {
    id: 27_967_122,
    puid: "wikidata/27967122",
    name: "Brian Postma SoundMon v2.x & v3.x module",
    extensions: &["bp3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
